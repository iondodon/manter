<script lang="ts">
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"
  import { IS_WINDOWS, IS_UNIX, PTY_WS_ADDRESS } from "../config/config"
  import { arrayBufferToString } from "../utils/utils"
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'

  export let sessionContext
  export let terminalInterface: Terminal
  export let ptyWebSocket: WebSocket
  export let fitAddon: FitAddon

  const openTerminalInterface = () => {
    if (!terminalInterface) {
      return
    }
    terminalInterface.open(document.getElementById('terminal'))
    sendProposedSizeToPty()
    adjustTerminalSize()
    terminalInterface.focus()
  }

  onMount(() => {
    openTerminalInterface()
    if (IS_UNIX) {
      setUser()
    }
  })

  const setUser = async () => {
    const settingsString = await invoke('get_settings') as string
    const settings = JSON.parse(settingsString)
    sessionContext['user'] = settings['default_login_user']
  }

  const sendProposedSizeToPty = () => {
    const proposedSize = fitAddon.proposeDimensions()
    const resizeData = {
        cols: proposedSize.cols, 
        rows: proposedSize.rows, 
        pixel_width: 0, 
        pixel_height: 0
    }
    ptyWebSocket.send(new TextEncoder().encode("\x01" + JSON.stringify(resizeData)))
  }

  const adjustTerminalSize = () => {
    fitAddon.fit()
    
    const terminal = document.getElementById("terminal") as HTMLElement
    const terminalHeight = terminal.clientHeight
    const terminalWidth = terminal.clientWidth

    const xtermElement = document.getElementsByClassName("xterm")[0] as HTMLElement
    const xtermViewPortElement = document.getElementsByClassName("xterm-viewport")[0] as HTMLElement

    xtermElement.style.height = `${terminalHeight}px`
    xtermElement.style.width = `${terminalWidth}px`

    xtermViewPortElement.style.height = `${terminalHeight}px`
    xtermViewPortElement.style.width = `${terminalWidth}px`
  }

  const termInterfaceHandleResize = (evt) => {
    const resizeValues = {
      cols: evt.cols, 
      rows: evt.rows, 
      pixel_width: 0, 
      pixel_height: 0
    }
    ptyWebSocket.send(new TextEncoder().encode("\x01" + JSON.stringify(resizeValues)))
    adjustTerminalSize()
  }

  const termInterfaceHandleUserInputData = (data: string) => {
    const encodedData = new TextEncoder().encode("\x00" + data)
    ptyWebSocket.send(encodedData)
  }

  const termInterfaceHandleTitleChange = (title) => {
    if (IS_UNIX && !sessionContext['isLoggedIn']) {
      openTerminalInterface()
      sessionContext['isLoggedIn'] = true
      adjustTerminalSize()
    }
    if (title.includes("[manter]")) {
      title = title.replace("[manter]", "")
      let promptUpdatedData = JSON.parse(title)
      sessionContext = {...sessionContext, ...promptUpdatedData}
      return
    }
    document.title = title
  }

  const setupNewTerminalInterface = () => {
    terminalInterface = new Terminal({
      cursorBlink: true,
      cursorStyle: 'bar',
      cursorWidth: 6
    })
    fitAddon = new FitAddon()
    terminalInterface.loadAddon(fitAddon)

    if (IS_WINDOWS) {
      openTerminalInterface()
      sendProposedSizeToPty()
      adjustTerminalSize()
    }

    addEventListener('resize', (_event) => adjustTerminalSize())
    terminalInterface.onResize((evt) => termInterfaceHandleResize(evt))
    terminalInterface.onData(data => termInterfaceHandleUserInputData(data))
    terminalInterface.onCursorMove(() => {})
    terminalInterface.buffer.onBufferChange(_buff => {})
    terminalInterface.onTitleChange(title => termInterfaceHandleTitleChange(title))
  }

  const handlePtyWsIncomingData = () => {
    if (IS_UNIX) {
      ptyWebSocket.onmessage = handlePtyCheckingPassword
      return
    }
    ptyWebSocket.onmessage = writePtyIncomingToTermInterface
  }

  const writePtyIncomingToTermInterface = (evt) => {
    if (!(evt.data instanceof ArrayBuffer)) {
      alert("unknown data type " + evt.data)
      return
    }
    const dataString: string = arrayBufferToString(evt.data.slice(1))
    terminalInterface.write(dataString)
    return dataString
  }

  const handlePtyCheckingPassword = async (evt) => {
    const writenData = writePtyIncomingToTermInterface(evt)
    console.log('checking pass')
    if (IS_UNIX && writenData.includes("Password:")) {
      tryLogin()
    }
  }

  const tryLogin = () => {
    const loginResultElement = document.getElementById('login-result') as HTMLDivElement
    loginResultElement.innerText = ""

    const password = (document.getElementById('password-input') as HTMLInputElement).value
    const loginData = { password: password}
    ptyWebSocket.send(new TextEncoder().encode("\x02" + JSON.stringify(loginData)))

    setTimeout(() => {
      if (sessionContext['isLoggedIn']) {
        terminalInterface.focus()
        ptyWebSocket.onmessage = writePtyIncomingToTermInterface
        return
      }
      loginResultElement.innerText = "Login failed"
      ptyWebSocket.close()
      console.log("WS closed because couldn't login")
    }, 2000)
  }

  const handlePtyWsClose = (_evt) => {
    terminalInterface.write("Session terminated")
    terminalInterface.dispose()
  }

  const handlePtyWsError = (evt) => {
    if (typeof console.log == "function") {
      console.log("ws error", evt)
    }
  }

  const newTerminalSession = async (_evt) => {
    ptyWebSocket = new WebSocket(PTY_WS_ADDRESS)
    ptyWebSocket.binaryType = "arraybuffer"

    ptyWebSocket.onopen = async (_evt) => {
      setupNewTerminalInterface()
      handlePtyWsIncomingData()
      ptyWebSocket.onclose = (evt) => handlePtyWsClose(evt)
      ptyWebSocket.onerror = (evt) => handlePtyWsError(evt)
    }
  }

  if (IS_WINDOWS && ptyWebSocket == null) {
    newTerminalSession(null)
  }

  const passInputOnKeyPress = (evt) => {
    if (evt.charCode === 13 && ptyWebSocket == null) {
      newTerminalSession(evt)
    }
  }
</script>

<div id="terminal">
  {#if IS_UNIX && !sessionContext['isLoggedIn']}
    <div id="login-form">
      <input 
        type="password" 
        id="password-input" 
        name="password" 
        required 
        minlength="4" 
        maxlength="20" 
        size="10"
        placeholder={`[sudo] password for ${sessionContext['user']}`}
        on:keypress={passInputOnKeyPress}
      >
      <br/>
      <button on:click={newTerminalSession} type="button">Login</button>
      <br/>
      <span id="login-result"></span>
    </div>
  {/if}
</div>

<style lang="scss">
  #terminal {
    display: flex;
    justify-content: center;
    align-items: center;
    
    top: 0em;
    right: 0em;
    height: calc(100% - 1.2em);
    width: 100%;

    background-color: black;
    overflow: hidden;
  }

  #login-form {
    width: 20rem;
    display: flex;
    min-height: fit-content;
    height: 30%;
    padding: 2rem;
    flex-direction: column;
    align-content: center;
    align-items: center;
    justify-content: center;
    color: hsl(0, 0%, 79%);
    background-color: hsl(0, 0%, 21%);
    border-radius: 0.4rem;
  }

  #login-form button, #login-form input {
    width: 100%;
  }

  #password-input {
    border: 1px solid hsl(0, 0%, 79%);
    border-radius: 0.3rem;
    padding: 0.35rem;
  }

  #login-form button {
    padding: 0.3rem;
  }

  #login-form input {
    text-align: center;
  }
</style>
