<script lang="ts">
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"
  import SuggestionsBox from "./SuggestionsBox.svelte"
  import { IS_WINDOWS, IS_UNIX, PTY_WS_ADDRESS } from "../config/config"
  import { ab2str } from "../utils/utils"

  let suggestionsBox: SuggestionsBox
  let sessionContext = {
    isLoggedIn: false,
    cwd: "~",
    password: ""
  }

  const adjustTerminalSize = (fitAddon) => {
    fitAddon.fit()
    
    const terminal = document.getElementById("terminal") as HTMLElement;
    const terminalHeight = terminal.clientHeight;
    const terminalWidth = terminal.clientWidth;

    const xtermElement = document.getElementsByClassName("xterm")[0] as HTMLElement;
    const xtermViewPortElement = document.getElementsByClassName("xterm-viewport")[0] as HTMLElement;

    xtermElement.style.height = `${terminalHeight}px`;
    xtermElement.style.width = `${terminalWidth}px`;

    xtermViewPortElement.style.height = `${terminalHeight}px`;
    xtermViewPortElement.style.width = `${terminalWidth}px`;
  }

  const login = (websocket: WebSocket) => {
    const password = (document.getElementById('password') as HTMLInputElement).value
    sessionContext['password'] = password
    const loginData = { password: password}
    websocket.send(new TextEncoder().encode("\x02" + JSON.stringify(loginData)))
  }

  const loginToNewTerminal = async (_evt) => {
    const websocket = new WebSocket(PTY_WS_ADDRESS)
    websocket.binaryType = "arraybuffer"

    websocket.onopen = async function(_evt) {
      const fitAddon: FitAddon = new FitAddon()
      const terminal: Terminal = new Terminal({
        cursorBlink: true,
        cursorStyle: 'bar',
        cursorWidth: 6
      })

      terminal.loadAddon(fitAddon)
      if (IS_WINDOWS) {
        terminal.open(document.getElementById('terminal'))
        adjustTerminalSize(fitAddon)
      }

      addEventListener('resize', (_event) => {
        adjustTerminalSize(fitAddon)
      })

      terminal.onResize(function(evt) {      
        const resizeData = {
            cols: evt.cols, 
            rows: evt.rows, 
            pixel_width: 4, 
            pixel_height: 4
        }
        websocket.send(new TextEncoder().encode("\x01" + JSON.stringify(resizeData)))
        adjustTerminalSize(fitAddon)
      })

      terminal.onScroll((_evt) => {
        
      })

      terminal.onData(async function(data: string) {
        if (suggestionsBox.isVisibleSuggestionsBox && suggestionsBox.filteredSuggestions.length > 0 && suggestionsBox.script.length > 0) {
          // if tab or enter
          if (data == "\t") {
            let nextText = suggestionsBox.takeFocusedSuggestion()
            nextText += ' '
            for (let i = 0; i < nextText.length; i++) {
              const encodedData = new TextEncoder().encode("\x00" + nextText[i])
              websocket.send(encodedData)
              await suggestionsBox.updateSuggestions(nextText[i], sessionContext)
            }
            return
          }
          // if esc
          if (data == "\x1b") {
            suggestionsBox.isVisibleSuggestionsBox = false
            return
          }
          // up
          if (data == "\x1b[A") {
            suggestionsBox.focusOnPrevSuggestion()
            return
          }
          // down
          if (data == "\x1b[B") {
            suggestionsBox.focusOnNextSuggestion()
            return
          }
        }

        const encodedData = new TextEncoder().encode("\x00" + data)
        websocket.send(encodedData)
        await suggestionsBox.updateSuggestions(data, sessionContext)
      })

      terminal.onCursorMove(() => {
        suggestionsBox.bringSuggestionsToCursor()
      })

      terminal.onKey(_evt => {
      
      })

      terminal.onSelectionChange(() => {
        
      })

      terminal.onBell(() => {
        console.log("bell")
      })

      terminal.buffer.onBufferChange((buf) => {console.log(buf.type)})

      terminal.onTitleChange(function(title) {
        if (IS_UNIX && !sessionContext['isLoggedIn']) {
          terminal.open(document.getElementById('terminal'))
          sessionContext['isLoggedIn'] = true
          adjustTerminalSize(fitAddon)
        }
        if (title.includes("[manter]")) {
            title = title.replace("[manter]", "")
            let promptUpdatedData = JSON.parse(title)
            sessionContext = {...sessionContext, ...promptUpdatedData}
            return
        }
        document.title = title
      })

      websocket.onmessage = async function(evt) {
        if (evt.data instanceof ArrayBuffer) {
          const data: string = ab2str(evt.data.slice(1))
          terminal.write(data)
          if (IS_UNIX && !sessionContext['isLoggedIn'] && data.includes("Password:")) {
            const loginResultElement = document.getElementById('login-result') as HTMLDivElement
            loginResultElement.innerText = ""
            login(websocket)
            setTimeout(() => {
              if (sessionContext['isLoggedIn']) {
                return
              }
              loginResultElement.innerText = "Login failed"
              websocket.close()
              console.log('WS closed')
            }, 1000)
          }
        } else {
          alert(evt.data)
        }
      }
 
      websocket.onclose = function(_evt) {
        terminal.write("Session terminated")
        terminal.dispose()
      }

      websocket.onerror = function(evt) {
        if (typeof console.log == "function") {
          console.log(evt)
        }
      }

    }
  }

  if (IS_WINDOWS) {
    loginToNewTerminal(null)
  }

  const passInputOnKeyPress = (evt) => {
    if (evt.charCode === 13) {
      loginToNewTerminal(null)
    }
  }

</script>

<div id="terminal">
    <SuggestionsBox bind:this={suggestionsBox} />
    {#if IS_UNIX && !sessionContext['isLoggedIn']}
       <div id="login-form">
        <input 
          type="password" 
          id="password" 
          name="password" 
          required 
          minlength="4" 
          maxlength="20" 
          size="10"
          placeholder="password"
          on:keypress={passInputOnKeyPress}
        >
        <br/>
        <button on:click={loginToNewTerminal} type="button">Login</button>
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
    width: 20%;
    display: inline-block;
    color: hsl(0, 0%, 79%);
  }

  #login-form button, #login-form input, #login-form span {
    margin: 0.3em;
    width: 100%;
  }

  #login-form input {
    text-align: center;
  }
</style>
