#![cfg(target_os = "windows")]

extern crate env_logger;
extern crate hydrogen;
extern crate simple_stream as ss;
use hydrogen;
use hydrogen::{Stream as HydrogenStream, HydrogenSocket};
use ss::frame::Frame;
use ss::frame::simple::{SimpleFrame, SimpleFrameBuilder};
use ss::{Socket, Plain, NonBlocking, SocketOptions};

use mt_logger::*;


#[derive(Clone)]
pub struct Stream {
    inner: Plain<Socket, SimpleFrameBuilder>
}

impl HydrogenStream for Stream {
    fn recv(&mut self) -> Result<Vec<Vec<u8>>, Error> {
        match self.inner.nb_recv() {
            Ok(frame_vec) => {
                let mut ret_buf = Vec::<Vec<u8>>::with_capacity(frame_vec.len());
                for frame in frame_vec.iter() {
                    ret_buf.push(frame.payload());
                }
                Ok(ret_buf)
            }
            Err(e) => Err(e)
        }
    }

    fn send(&mut self, buf: &[u8]) -> Result<(), Error> {
        let frame = SimpleFrame::new(buf);
        self.inner.nb_send(&frame)
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        self.inner.shutdown()
    }
}
impl AsRawFd for Stream {
    fn as_raw_fd(&self) -> RawFd { self.inner.as_raw_fd() }
}


struct Server;
impl hydrogen::Handler for Server {
    fn on_server_created(&mut self, fd: RawFd) {
        mt_log!(Level::Info, "on_server_created: {}", fd);
    }

    fn on_new_connection(&mut self, fd: RawFd) -> Arc<UnsafeCell<HydrogenStream>> {

    }

    fn on_data_received(&mut self, socket: HydrogenSocket, buffer: Vec<u8>) {

    }

    fn on_connection_removed(&mut self, fd: RawFd, err: Error) {

    }
}


fn main() {
    hydrogen::begin(Server, hydrogen::Config {
        addr: "0.0.0.0".to_string(),
        port: 1337,
        max_threads: 8,
        pre_allocated: 100000
    });
}
