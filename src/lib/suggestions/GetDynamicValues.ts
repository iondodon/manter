import { Child, Command } from "@tauri-apps/api/shell"

const windows = navigator.userAgent.includes('Windows')
let cmd = windows ? 'cmd' : 'sh'
let args = windows ? ['/C'] : ['-c']
let env = {
  // key: val
}
let child: Child

function executeScript(wrapper, sessionContext, script): Promise<{code: any, res: any[]}> {
  const cwd = sessionContext['cwd']
  let res = []

  child = null
  const command = new Command(cmd, [...args, script], { cwd: cwd || null, env: env })

  command.stdout.on('data', line => {
    res.push(wrapper['postProcessor'](line))
  })
  
  command.spawn()
    .then(c => child = c)
    .catch(r => console.log(r))

  command.stderr.on('data', line => {
    console.log('stderr ' + line)
  })

  return new Promise((resolve, reject) => {
    command.on('close', data => {
      console.log(`Command finished with code ${data.code} and signal ${data.signal}`)
      child = null
      if (data.code != 0) {
        reject(`Command finished with code ${data.code} and signal ${data.signal}`)
      }
      resolve({code: data.code, res: res})
    })

    command.on('error', error => {
      console.log('error', error)
      reject("error " + error)
    })
  })
}

function tryNonSudo(wrapper, sessionContext): Promise<{code: any, res: any[]}> {
  const script = wrapper['script']
  return executeScript(wrapper, sessionContext, script)
}

function trySudo(wrapper, sessionContext): Promise<{code: any, res: any[]}> {
  const script = ' echo "' + sessionContext['password'] + '" | sudo -S ' + wrapper['script'] + '; sudo -K;'
  return executeScript(wrapper, sessionContext, script)
}

export function getDynamicValues(wrapper, sessionContext): Promise<any[]> {
  return new Promise((resolve, reject) => {
    tryNonSudo(wrapper, sessionContext)
      .then(responseTryNonSudo => resolve(responseTryNonSudo.res))
      .catch(errResponseTryNonSudo => {
        console.log("Try sudo, because of " + JSON.stringify(errResponseTryNonSudo))
        trySudo(wrapper, sessionContext)
          .then(responseTrySudo => resolve(responseTrySudo.res))
          .catch(errResponseTrySudo => reject(errResponseTrySudo))
      })
  })
}

function kill() {
  child.kill().then(() => console.log('killed child process')).catch((r: any) => console.log(r))
}

function writeToStdin(toWrite) {
  child.write(toWrite).catch((r: any) => console.log(r))
}

