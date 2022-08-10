<script lang="ts">

  import { onMount } from "svelte"
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"
  import SuggestionsBox from "./SuggestionsBox.svelte"
  import { IS_WINDOWS, PTY_WS_ADDRESS } from "../config/config"
  import { ab2str } from "../utils/utils"

  let isLoggedIn = false
  let suggestionsBox: SuggestionsBox;
  let promptContext = {
    cwd: "~"
  }

  const login = (websocket: WebSocket, password) => {
    const loginData = { password: password}
    websocket.send(new TextEncoder().encode("\x02" + JSON.stringify(loginData)))
  }

  const setCanvasSize = (fitAddon) => {
    fitAddon.fit()
    if (isLoggedIn) {
      const canvasCursorLayerElement = document.getElementsByClassName('xterm-viewport')[0] as HTMLElement
      const terminalElement = document.getElementById('terminal')
      canvasCursorLayerElement.style.width = `${terminalElement.offsetWidth}px`
      canvasCursorLayerElement.style.height = `${terminalElement.offsetHeight}px`
    }
  }

  const start = async (_evt) => {
    const passwordElement = document.getElementById('password') as HTMLInputElement
    const password = passwordElement.value

    const websocket = new WebSocket(PTY_WS_ADDRESS)
    websocket.binaryType = "arraybuffer"

    setTimeout(() => {
      const loginResultElement = document.getElementById('login-result') as HTMLDivElement
      if (!isLoggedIn) {
        loginResultElement.innerText = "Login failed"
        websocket.close()
        return
      }
    }, 5000)

    websocket.onopen = async function(_evt) {
      const fitAddon: FitAddon = new FitAddon()
      const terminal: Terminal = new Terminal({
        cursorBlink: true,
        cursorStyle: 'bar',
        cursorWidth: 6
      })

      terminal.loadAddon(fitAddon)

      if (!IS_WINDOWS) {
        login(websocket, password)
      }

      setCanvasSize(fitAddon)
      addEventListener('resize', (_event) => {
        setCanvasSize(fitAddon)
      })

      terminal.onData(async function(data: string) {
        if (suggestionsBox.isVisible && suggestionsBox.filteredSuggestions.length > 0 && suggestionsBox.script.length > 0) {
          // if tab or enter
          if (data == "\t") {
            let nextText = suggestionsBox.takeSuggestion()
            nextText += ' '
            for (let i = 0; i < nextText.length; i++) {
              const encodedData = new TextEncoder().encode("\x00" + nextText[i])
              websocket.send(encodedData)
              await suggestionsBox.updateSuggestions(nextText[i], promptContext)
            }
            return
          }
          // if esc
          if (data == "\x1b") {
            suggestionsBox.isVisible = false
            return
          }
          // up
          if (data == "\x1b[A") {
            suggestionsBox.selectPrevSuggestion()
            return
          }
          // down
          if (data == "\x1b[B") {
            suggestionsBox.selectNextSuggestion()
            return
          }
        }

        const encodedData = new TextEncoder().encode("\x00" + data)
        websocket.send(encodedData)
        await suggestionsBox.updateSuggestions(data, promptContext)
      })

      terminal.onCursorMove(() => {
        suggestionsBox.bringSuggestionsToCursor()
      })

      terminal.onResize(function(evt) {      
        setCanvasSize(fitAddon)
        const resizeData = {
            cols: evt.cols, 
            rows: evt.rows, 
            pixel_width: 0, 
            pixel_height: 0
        }
        websocket.send(new TextEncoder().encode("\x01" + JSON.stringify(resizeData)))
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
        if (!isLoggedIn && !IS_WINDOWS) {
          terminal.open(document.getElementById('terminal'))
          isLoggedIn = true
        }
        if (title.includes("[manter]")) {
            title = title.replace("[manter]", "")
            promptContext = JSON.parse(title)
            return
        }
        document.title = title
      })

      websocket.onmessage = async function(evt) {
        if (evt.data instanceof ArrayBuffer) {
          const data = ab2str(evt.data.slice(1))
          terminal.write(data)
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

</script>

<div id="terminal">
    <SuggestionsBox bind:this={suggestionsBox} />
</div>

{#if !IS_WINDOWS && !isLoggedIn}
  <div id="login-form">
    <label for="name">Password:</label>
    <input type="text" id="password" name="password" required minlength="4" maxlength="20" size="10">
    <button on:click={start} type="button">Login</button>
    <br/>
    <span id="login-result"></span>
  </div>
{/if}

<style lang="scss">
  #terminal {
    width: 100%;
    height: 100%;
    padding: 0;
    margin: 0;
  }

  #login-form {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    text-align: center;
  }
</style>
