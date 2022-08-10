import { Command } from "@tauri-apps/api/shell"

const windows = navigator.userAgent.includes('Windows')
let cmd = windows ? 'cmd' : 'sh'
let args = windows ? ['/C'] : ['-c', 'SUDO_ASKPASS=/usr/bin/ssh-askpass', 'sudo', '-A']

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

export function getDynamicValues(wrapper, cwd): Promise<any[]> {
  let res = []

  child = null
  const command = new Command(cmd, [...args, wrapper['script']], { cwd: cwd || null, env: _getEnv() })
  
  command.stdout.on('data', line => {
    res.push(wrapper['postProcessor'](line))
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

      command.on('error', error => {
        console.log(error)
        reject(error)
      })
      command.stderr.on('data', line => {
        console.log(line)
        reject(line)
      })
  })
}

function kill() {
  child.kill().then(() => console.log('killed child process')).catch((r: any) => console.log(r))
}

function writeToStdin() {
  child.write(stdin).catch((r: any) => console.log(r))
}

