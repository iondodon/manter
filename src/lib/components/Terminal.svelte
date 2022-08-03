<script lang="ts">

  import { onMount } from "svelte"
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"
  import SuggestionsBox from "./SuggestionsBox.svelte"
  import { IS_WINDOWS, PTY_WS_ADDRESS } from "../config/config"
  import { ab2str } from "../utils/utils"

  let suggestionsBox: SuggestionsBox;
  let promptContext = {
    cwd: "~"
  }

  const setupShell = (websocket: WebSocket) => {
    const setupData = {
      password: "eronat98"
    }
    websocket.send(new TextEncoder().encode("\x02" + JSON.stringify(setupData)))
  }

  onMount(async () => {
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

      terminal.open(document.getElementById('terminal'))
      fitAddon.fit()

      addEventListener('resize', (_event) => {
        fitAddon.fit()
      })

      terminal.onData(async function(data: string) {
        if (suggestionsBox.isVisible) {
          // if tab or enter
          if ((data == "\t" || data == "\r") && suggestionsBox.filteredSuggestions.length > 0) {
            let nextText = suggestionsBox.takeSuggestion()
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
        const resizeData = {
            cols: evt.cols, 
            rows: evt.rows, 
            pixel_width: 0, 
            pixel_height: 0
        }
        websocket.send(new TextEncoder().encode("\x01" + JSON.stringify(resizeData)))
      })

      terminal.onKey(evt => {
      
      })

      terminal.onSelectionChange(() => {
        
      })

      terminal.onBell(() => {
        console.log("bell")
      })

      terminal.buffer.onBufferChange((buf) => {console.log(buf.type)})

      terminal.onTitleChange(function(title) {
        console.log('title change', title)
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

      if (!IS_WINDOWS) {
        setupShell(websocket)
      }
    }
  })

</script>

<div>
  <div id="terminal">
    <SuggestionsBox bind:this={suggestionsBox} />
  </div>
</div>

<style lang="scss">
  #terminal {
    width: 100%;
    height: 100%;
    margin: 0%;
    padding: 0%;
  }
</style>
