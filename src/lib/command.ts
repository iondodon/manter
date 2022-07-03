import { Command } from "@tauri-apps/api/shell"

const windows = navigator.userAgent.includes('Windows')
let cmd = windows ? 'cmd' : 'sh'
let args = windows ? ['/C'] : ['-c']

// let adHockScript = 'echo "hello world"'
let cwd = null
let env = 'SOMETHING=value ANOTHER=2'
let stdin = ''
let child

function _getEnv() {
  return env.split(' ').reduce((env, clause) => {
    let [key, value] = clause.split('=')
    return {
    ...env,
    [key]: value
    }
  }, {})
}

export function spawn(adHockScript) {
  child = null
  const command = new Command(cmd, [...args, adHockScript], { cwd: cwd || null, env: _getEnv() })
  command.on('close', data => {
    console.log(`command finished with code ${data.code} and signal ${data.signal}`)
    child = null
  })
  command.on('error', error => console.log(`command error: "${error}"`))
  command.stdout.on('data', line => console.log(`command stdout: "${line}"`))
  command.stderr.on('data', line => console.log(`command stderr: "${line}"`))
  
  command.spawn()
    .then(c => {
      child = c
    })
    .catch(r => console.log(r))
}

function kill() {
  child.kill().then(() => console.log('killed child process')).catch((r: any) => console.log(r))
}

function writeToStdin() {
  child.write(stdin).catch((r: any) => console.log(r))
}

