use core::pin::Pin;
use core::task::{Context, Poll};
use std::ffi::{CStr, OsStr};
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::io::RawFd;
use std::os::unix::prelude::*;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::io::unix::AsyncFd;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, Error, ReadBuf};
use tokio::sync::mpsc;

pub struct PtyMaster {
    inner: Arc<AsyncFd<File>>,
    closed: Arc<AtomicBool>,
    slave: Option<File>,
}

impl Clone for PtyMaster {
    fn clone(&self) -> Self {
        PtyMaster {
            inner: self.inner.clone(),
            closed: self.closed.clone(),
            slave: self.slave.as_ref().map(|s| s.try_clone().unwrap()),
        }
    }
}

impl PtyMaster {
    pub fn new() -> Result<Self, std::io::Error> {
        let inner = unsafe {
            let fd = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);

            if fd < 0 {
                return Err(std::io::Error::last_os_error());
            }

            if libc::grantpt(fd) != 0 {
                return Err(std::io::Error::last_os_error());
            }

            if libc::unlockpt(fd) != 0 {
                return Err(std::io::Error::last_os_error());
            }

            let flags = libc::fcntl(fd, libc::F_GETFL, 0);
            if flags < 0 {
                return Err(std::io::Error::last_os_error());
            }

            if libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) == -1 {
                return Err(std::io::Error::last_os_error());
            }

            AsyncFd::new(std::fs::File::from_raw_fd(fd))?
        };
        Ok(PtyMaster {
            inner: Arc::new(inner),
            closed: Arc::new(AtomicBool::new(false)),
            slave: None,
        })
    }

    pub fn open_sync_pty_slave(&mut self) -> Result<File, std::io::Error> {
        let mut buf: [libc::c_char; 512] = [0; 512];
        let fd = self.as_raw_fd();

        if unsafe { libc::ptsname_r(fd, buf.as_mut_ptr(), buf.len()) } != 0 {
            return Err(std::io::Error::last_os_error());
        }

        let ptsname = OsStr::from_bytes(unsafe { CStr::from_ptr(&buf as _) }.to_bytes());
        match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(ptsname)
        {
            Ok(slave) => {
                self.slave.replace(slave.try_clone().unwrap());
                Ok(slave)
            }
            Err(e) => Err(e),
        }
    }

    pub fn resize(
        &mut self,
        cols: libc::c_ushort,
        lines: libc::c_ushort,
    ) -> Result<(), std::io::Error> {
        let fd = self.as_raw_fd();
        let winsz = libc::winsize {
            ws_row: lines,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        if unsafe { libc::ioctl(fd, libc::TIOCSWINSZ.into(), &winsz) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}

impl AsRawFd for PtyMaster {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.get_ref().as_raw_fd()
    }
}

impl AsyncRead for PtyMaster {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let b =
            unsafe { &mut *(buf.unfilled_mut() as *mut [std::mem::MaybeUninit<u8>] as *mut [u8]) };
        loop {
            let closed = self.closed.load(core::sync::atomic::Ordering::SeqCst);
            if closed {
                return Poll::Ready(Ok(()));
            }
            let mut g = match self.inner.poll_read_ready(cx) {
                Poll::Ready(Ok(g)) => g,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            };
            let f = self.inner.clone();
            match f.get_ref().read(b) {
                Ok(s) => {
                    if s.gt(&0) {
                        unsafe {
                            buf.assume_init(s);
                            buf.advance(s);
                        }
                    }
                    return Poll::Ready(Ok(()));
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        g.clear_ready();
                    }
                    _ => {
                        return Poll::Ready(Err(e));
                    }
                },
            }
        }
    }
}

impl AsyncWrite for PtyMaster {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        loop {
            let mut g = match self.inner.poll_write_ready(cx) {
                Poll::Ready(Ok(g)) => g,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            };
            let f = self.inner.clone();
            match f.get_ref().write(buf) {
                Ok(s) => return Poll::Ready(Ok(s)),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        g.clear_ready();
                    }
                    _ => return Poll::Ready(Err(e)),
                },
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        loop {
            let mut g = match self.inner.poll_write_ready(cx) {
                Poll::Ready(Ok(g)) => g,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Pending => return Poll::Pending,
            };
            let f = self.inner.clone();
            match f.get_ref().flush() {
                Ok(()) => return Poll::Ready(Ok(())),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::WouldBlock => {
                        g.clear_ready();
                    }
                    _ => return Poll::Ready(Err(e)),
                },
            }
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        if self
            .closed
            .compare_and_swap(false, true, core::sync::atomic::Ordering::SeqCst)
        {
            return Poll::Ready(Ok(()));
        }
        if let Some(ref mut slave) = self.slave {
            slave.write(&[0])?;
        }
        Poll::Ready(Ok(()))
    }
}

use tokio::process::Command;

pub struct PtyCommand {
    inner: Command,
}

impl From<Command> for PtyCommand {
    fn from(c: Command) -> Self {
        PtyCommand { inner: c }
    }
}

impl PtyCommand {
    pub async fn run(
        &mut self,
        mut stopper: mpsc::UnboundedReceiver<()>,
    ) -> Result<PtyMaster, std::io::Error> {
        let mut pty_master = PtyMaster::new().unwrap();
        let master_fd = pty_master.as_raw_fd();
        let slave = pty_master.open_sync_pty_slave().unwrap();
        self.inner
            .stdin(slave.try_clone().unwrap())
            .stdout(slave.try_clone().unwrap())
            .stderr(slave.try_clone().unwrap());

        unsafe {
            self.inner.pre_exec(move || {
                if libc::close(master_fd) != 0 {
                    return Err(std::io::Error::last_os_error());
                }

                if libc::setsid() < 0 {
                    return Err(std::io::Error::last_os_error());
                }

                if libc::ioctl(0, libc::TIOCSCTTY.into(), 1) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }

        let mut child = self.inner.spawn()?;
        let mut master_cl = pty_master.clone();
        let fut = async move {
            let _ = tokio::select! {
                _exit_st = (&mut child).wait() => (),
                _ = stopper.recv() => {
                    let _ = (&mut child).start_kill().map_err(|e| {
                        log::error!("failed to kill pty child: {:?}", e);
                    });
                    let _ = (&mut child).wait().await.map_err(|e| {
                        log::error!("kill wait pty child error: {:?}", e);
                    });
                },
            };
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            master_cl.shutdown().await?;
            Ok::<(), anyhow::Error>(())
        };
        tokio::spawn(fut);
        Ok(pty_master)
    }
}
