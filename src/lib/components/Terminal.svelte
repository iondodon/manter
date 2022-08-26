<script lang="ts">
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"
  import SuggestionsBox from "./SuggestionsBox.svelte"
  import { IS_WINDOWS, IS_UNIX, PTY_WS_ADDRESS } from "../config/config"
  import { arrayBufferToString } from "../utils/utils"
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'

  let suggestionsBox: SuggestionsBox
  let sessionContext = {
    isLoggedIn: false,
    cwd: "~",
    password: "",
    user: ""
  }

  let ptyWebSocket: WebSocket
  let fitAddon: FitAddon
  let terminalInterface: Terminal
  let bufferType = "normal"

  const setUser = async () => {
    const settingsString = await invoke('get_settings') as string
    const settings = JSON.parse(settingsString)
    sessionContext['user'] = settings['default_login_user']
  }

  onMount(() => {
    if (IS_UNIX) {
      setUser()
    }
  })

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

  const focusOnTerminal = () => {
    terminalInterface.focus()
  }

  const tryLogin = () => {
    const loginResultElement = document.getElementById('login-result') as HTMLDivElement
    loginResultElement.innerText = ""

    const password = (document.getElementById('password-input') as HTMLInputElement).value
    sessionContext['password'] = password
    const loginData = { password: password}
    ptyWebSocket.send(new TextEncoder().encode("\x02" + JSON.stringify(loginData)))

    setTimeout(() => {
      if (sessionContext['isLoggedIn']) {
        focusOnTerminal()
        ptyWebSocket.onmessage = writePtyIncomingToTermInterface
        return
      }
      loginResultElement.innerText = "Login failed"
      ptyWebSocket.close()
      console.log("WS closed because couldn't login")
    }, 2000)
  }

  const termInterfaceHandleResize = () => {
    addEventListener('resize', (_event) => {
      adjustTerminalSize()
      suggestionsBox.bringSuggestionsToCursor()
    })
    terminalInterface.onResize((evt) => {      
      const resizeValues = {
          cols: evt.cols, 
          rows: evt.rows, 
          pixel_width: 0, 
          pixel_height: 0
      }
      ptyWebSocket.send(new TextEncoder().encode("\x01" + JSON.stringify(resizeValues)))
      adjustTerminalSize()
    })
  }

  const termInterfaceHandleUserInputData = () => {
    terminalInterface.onData(async (data: string) => {
      if (bufferType == "normal"
            && suggestionsBox.isVisibleSuggestionsBox 
            && suggestionsBox.filteredSuggestions.length > 0 
            && suggestionsBox.script.length > 0) {
        // if tab or enter
        if (data == "\t") {
          let nextText = suggestionsBox.takeFocusedSuggestion()
          for (let i = 0; i < nextText.length - 1; i++) {
            const encodedData = new TextEncoder().encode("\x00" + nextText[i])
            ptyWebSocket.send(encodedData)
            await suggestionsBox.updateSuggestions(nextText[i], sessionContext, true)
          }

          const encodedData = new TextEncoder().encode("\x00" + nextText[nextText.length - 1])
          ptyWebSocket.send(encodedData)
          await suggestionsBox.updateSuggestions(nextText[nextText.length - 1], sessionContext, false)
          
          const spaceData = new TextEncoder().encode("\x00" + ' ')
          ptyWebSocket.send(spaceData)
          await suggestionsBox.updateSuggestions(' ', sessionContext, false)

          return
        }
        // if esc
        if (data == "\x1b") {
          suggestionsBox.isVisibleSuggestionsBox = false
          return
        }
        // up
        if (data == "\x1b[A" || data == "\u001bOA") {
          suggestionsBox.focusOnPrevSuggestion()
          return
        }
        // down
        if (data == "\x1b[B" || data == "\u001bOB") {
          suggestionsBox.focusOnNextSuggestion()
          return
        }
      }

      const encodedData = new TextEncoder().encode("\x00" + data)
      ptyWebSocket.send(encodedData)

      if (bufferType == "normal") {
        await suggestionsBox.updateSuggestions(data, sessionContext, false)
      }
    })
  }

  const termInterfaceHandleCursorMove = () => {
    terminalInterface.onCursorMove(() => {

    })
  }

  const termInterfaceHandleBufferChange = () => {
    terminalInterface.buffer.onBufferChange((buff) => {
      bufferType = buff.type
    })
  }

  const linkTermInterfaceToHtmlElement = () => {
    terminalInterface.open(document.getElementById('terminal'))
    termInterfaceHandleViewportScroll()
  }

  const setLoggedIn = () => {
    if (!sessionContext['isLoggedIn']) {
      linkTermInterfaceToHtmlElement()
      sessionContext['isLoggedIn'] = true
      adjustTerminalSize()
    }
  }

  const updateSessionContext = (title) => {
    title = title.replace("[manter]", "")
    let promptUpdatedData = JSON.parse(title)
    sessionContext = {...sessionContext, ...promptUpdatedData}
  }

  const termInterfaceHandleTitleChange = () => {
    terminalInterface.onTitleChange((title) => {
      if (IS_UNIX) {
        setLoggedIn()
      }
      if (title.includes("[manter]")) {
          updateSessionContext(title)
          return
      }
      document.title = title
    })
  }

  const termInterfaceHandleViewportScroll = () => {
    const viewport = document.querySelector('.xterm-viewport') as HTMLElement
    viewport.addEventListener('scroll', (_evt) => {
      suggestionsBox.isVisibleSuggestionsBox = false
    })
  }

  const createNewTermInterface = () => {
    terminalInterface = new Terminal({
      cursorBlink: true,
      cursorStyle: 'bar',
      cursorWidth: 6
    })
    fitAddon = new FitAddon()
    terminalInterface.loadAddon(fitAddon)
  }

  const setupNewTerminalInterface = () => {
    createNewTermInterface()

    if (IS_WINDOWS) {
      linkTermInterfaceToHtmlElement()
      sendProposedSizeToPty()
      adjustTerminalSize()
    }

    termInterfaceHandleResize()
    termInterfaceHandleUserInputData()
    termInterfaceHandleCursorMove()
    termInterfaceHandleBufferChange()
    termInterfaceHandleTitleChange()
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
    if (IS_UNIX && !sessionContext['isLoggedIn'] && writenData.includes("Password:")) {
      tryLogin()
    }
  }

  const handlePtyWsIncomingData = () => {
    if (IS_UNIX) {
      ptyWebSocket.onmessage = handlePtyCheckingPassword
      return
    }
    ptyWebSocket.onmessage = writePtyIncomingToTermInterface
  }

  const handlePtyWsClose = () => {
    ptyWebSocket.onclose = (_evt) => {
      terminalInterface.write("Session terminated")
      terminalInterface.dispose()
    }
  }

  const handlePtyWsError = () => {
    ptyWebSocket.onerror = (evt) => {
      if (typeof console.log == "function") {
        console.log("ws error", evt)
      }
    }
  }

  const newTerminalSession = async (_evt) => {
    if (ptyWebSocket) {
      ptyWebSocket.close()
    }
    ptyWebSocket = new WebSocket(PTY_WS_ADDRESS)
    ptyWebSocket.binaryType = "arraybuffer"

    ptyWebSocket.onopen = async (_evt) => {
      setupNewTerminalInterface()
      handlePtyWsIncomingData()
      handlePtyWsClose()
      handlePtyWsError()
    }
  }

  if (IS_WINDOWS) {
    newTerminalSession(null)
  }

  const passInputOnKeyPress = (evt) => {
    if (evt.charCode === 13) {
      newTerminalSession(evt)
    }
  }
</script>

<div id="terminal">
    <SuggestionsBox bind:this={suggestionsBox} />
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
