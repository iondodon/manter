import { Command } from "@tauri-apps/api/shell"

const windows = navigator.userAgent.includes('Windows')
let cmd = windows ? 'cmd' : 'sh'
let args = windows ? ['/C'] : ['-c']

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

export function getDynamicValues(wrapper, promptContext): Promise<any[]> {
  const cwd = promptContext['cwd']
  const pass = promptContext['password']
  let res = []

  child = null
  const script = "echo \"" + pass + "\" | sudo -S " + wrapper['script'] + "; sudo -K;"
  const command = new Command(cmd, [...args, script], { cwd: cwd || null, env: _getEnv() })

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
        reject("error " + error)
      })
  })
}

function kill() {
  child.kill().then(() => console.log('killed child process')).catch((r: any) => console.log(r))
}

function writeToStdin() {
  child.write(stdin).catch((r: any) => console.log(r))
}

