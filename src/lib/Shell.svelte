<script>

  import { onMount } from 'svelte'
  import { Terminal } from "xterm"
  import { FitAddon } from 'xterm-addon-fit'
  import "xterm/css/xterm.css"

  onMount(async () => {
    var term;
    var websocket = new WebSocket("ws://127.0.0.1:7703" );
    websocket.binaryType = "arraybuffer";


    function ab2str(buf) {
      return String.fromCharCode.apply(null, new Uint8Array(buf));
    }

    websocket.onopen = function(evt) {
      const fitAddon = new FitAddon()

      term = new Terminal({
        // screenKeys: true,
        // useStyle: true,
        cursorBlink: true,
        cursorStyle: 'bar',
        cursorWidth: 6
      });

      term.loadAddon(fitAddon)

      term.onData(function(data) {
        websocket.send(new TextEncoder().encode("\x00" + data));
      });

      term.onResize(function(evt) {
        websocket.send(new TextEncoder().encode("\x01" + JSON.stringify({cols: evt.cols, rows: evt.rows})))
      });

      term.buffer.onBufferChange((buf) => {console.log(buf.type)})

      term.onTitleChange(function(title) {
        document.title = title;
      });

      term.open(document.getElementById('terminal'));
        fitAddon.fit()
        websocket.onmessage = function(evt) {
        if (evt.data instanceof ArrayBuffer) {
          term.write(ab2str(evt.data.slice(1)));
        } else {
          alert(evt.data)
        }
      }

      websocket.onclose = function(evt) {
        term.write("Session terminated");
        term.destroy();
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

<style>

</style>
