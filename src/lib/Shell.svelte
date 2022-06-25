<script lang="ts">

  import { onMount } from 'svelte'
  import { Terminal }  from 'xterm'
  import { FitAddon }   from 'xterm-addon-fit'
  import "xterm/css/xterm.css"

  onMount(async () => {
    const websocket = new WebSocket("ws://127.0.0.1:7703" )
    websocket.binaryType = "arraybuffer"


    function ab2str(buf: ArrayBuffer) {
      return String.fromCharCode.apply(null, new Uint8Array(buf))
    }

    websocket.onopen = function(evt) {
      console.log(evt)

      const fitAddon: FitAddon = new FitAddon()

      const terminal: Terminal = new Terminal({
        cursorBlink: true,
        cursorStyle: 'bar',
        cursorWidth: 6
      })

      terminal.loadAddon(fitAddon)

      terminal.onData(function(data) {
        let encodedData = new TextEncoder().encode("\x00" + data)
        console.log(encodedData)
        websocket.send(encodedData)
      })

      terminal.onResize(function(evt) {
        websocket.send(new TextEncoder().encode("\x01" + JSON.stringify({cols: evt.cols, rows: evt.rows})))
      })

      terminal.buffer.onBufferChange((buf) => {console.log(buf.type)})

      terminal.onTitleChange(function(title) {
        document.title = title
      })

      const htmlTerminal = document.getElementById('terminal')
      terminal.open(htmlTerminal)

      fitAddon.fit()

      websocket.onmessage = function(evt) {
        if (evt.data instanceof ArrayBuffer) {
          terminal.write(ab2str(evt.data.slice(1)))
        } else {
          alert(evt.data)
        }
      }

      websocket.onclose = function(evt) {
        console.log(evt)
        terminal.write("Session terminated")
        terminal.dispose()
      }

      websocket.onerror = function(evt) {
        if (typeof console.log == "function") {
          console.log(evt)
        }
      }
    }
  })

</script>

<div>
  <div id="terminal"></div>
</div>

<style lang="scss">

</style>
