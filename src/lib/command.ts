import { Command } from "@tauri-apps/api/shell"

const windows = navigator.userAgent.includes('Windows')
let cmd = windows ? 'cmd' : 'sh'
let args = windows ? ['/C'] : ['-c']

let env = 'SOMETHING=value ANOTHER=2'
let stdin = ''
let child
let logged = false

function _getEnv() {
  return env.split(' ').reduce((env, clause) => {
    let [key, value] = clause.split('=')
    return {
    ...env,
    [key]: value
    }
  }, {})
}

export function spawn(wrapper, cwd) {
  let res = []

  // if (!logged) {
  //   child = null
  //   const cmdd = new Command(cmd, [...args, ''], { cwd: null || null, env: _getEnv() })
  //   cmdd.stderr.on('data', line => console.log(line))
  //   cmdd.on('close', data => {
  //     console.log(`command finished with code ${data.code} and signal ${data.signal}`)
  //     child = null
      
  //   })
  //   cmdd.on('error', error => console.log(error))
  //   cmdd.spawn()
  //   .then(c => {
  //     child = c
  //   })
  //   .catch(r => console.log(r))
  //   // logged = true
  // }

  child = null
  console.log("cwd", cwd)
  const command = new Command(cmd, [...args, `cd ${cwd};` + wrapper['script']], { cwd: null || null, env: _getEnv() })
  
  command.stdout.on('data', line => {
    res.push(wrapper['processor'](line))
  })
  
  command.spawn()
    .then(c => {
      child = c
    })
    .catch(r => console.log(r))

  return new Promise((resolve, reject) => {
      command.on('close', data => {
        console.log(`command finished with code ${data.code} and signal ${data.signal}`)
        child = null
        resolve(res)
      })

      command.on('error', error => reject(error))
      command.stderr.on('data', line => reject(line))
  })
}

function kill() {
  child.kill().then(() => console.log('killed child process')).catch((r: any) => console.log(r))
}

function writeToStdin() {
  child.write(stdin).catch((r: any) => console.log(r))
}

