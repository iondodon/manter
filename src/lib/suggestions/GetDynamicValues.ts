import { Command } from "@tauri-apps/api/shell"
import { IS_MACINTOSH } from "../config/config"

const windows = navigator.userAgent.includes('Windows')
let cmd = windows ? 'cmd' : 'sh'
let args = windows ? ['/C'] : ['-c']

let SUDO_ASKPASS = 'SUDO_ASKPASS=/usr/bin/ssh-askpass'
if (IS_MACINTOSH) {
  SUDO_ASKPASS = 'SUDO_ASKPASS=/usr/local/bin/ssh-askpass'
}
let env = `${SUDO_ASKPASS}`

let child

function getEnv() {
  return env.split(' ').reduce((env, clause) => {
    let [key, value] = clause.split('=')
    return {
    ...env,
    [key]: value
    }
  }, {})
}

export function getDynamicValues(wrapper, sessionContext): Promise<any[]> {
  const cwd = sessionContext['cwd']
  let res = []

  child = null

  let script = wrapper['script']
  if (script.startsWith('sudo -S')) {
    script = 'echo "' + sessionContext['password'] + '" | ' + script + '; sudo -K'
  }

  const command = new Command(cmd, [...args, wrapper['script']], { cwd: cwd || null, env: getEnv() })

  command.stdout.on('data', line => {
    res.push(wrapper['postProcessor'](line))
  })
  
  command.spawn()
    .then(c => {
      child = c
    })
    .catch(r => console.log(r))

  command.stderr.on('data', line => {
    console.log('stderr ' + line)
  })

  return new Promise((resolve, reject) => {
    command.on('close', data => {
      console.log(`command finished with code ${data.code} and signal ${data.signal}`)
      child = null
      resolve(res)
    })

    command.on('error', error => {
      console.log('error', error)
      reject("error " + error)
    })
  })
}

function kill() {
  child.kill().then(() => console.log('killed child process')).catch((r: any) => console.log(r))
}

function writeToStdin(toWrite) {
  child.write(toWrite).catch((r: any) => console.log(r))
}

