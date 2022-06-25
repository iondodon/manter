<script lang="ts">

  import { onMount } from "svelte"
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"

  let script: string = ''

  onMount(async () => {
    const websocket = new WebSocket("ws://127.0.0.1:7703")
    websocket.binaryType = "arraybuffer"


    function ab2str(buf: ArrayBuffer) {
      return String.fromCharCode.apply(null, new Uint8Array(buf))
    }

    websocket.onopen = function(_evt) {
      const fitAddon: FitAddon = new FitAddon()
      const terminal: Terminal = new Terminal({
        cursorBlink: true,
        cursorStyle: 'bar',
        cursorWidth: 6
      })

      terminal.loadAddon(fitAddon)


      terminal.onData(function(data: string) {
        handleScript(data)
        let encodedData = new TextEncoder().encode("\x00" + data)
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
  })

  const getLastWordsFromScript = (script: string): string => {
    const words = script.trim().split(' ')
    return words[words.length - 1]
  }


  const files: object = {
    'file': () => {
      return 'Get file script'
    },
    'next': [this]
  }

  const lsOptions: object = {
    '-a': {
      'name': 'all',
      'description': 'ls all - description',
      'next': [this, files]
    }
  }

  const commands: object = {
    'ls': {
      'name': "ls",
      'description': "ls description",
      'next': [lsOptions, files]
    }
  }

  let next: any = [commands]
  const handleScript = (data: string) => {
    if (data.length < 0) {
      return
    }
    if (data === '\n') {
      script = ''
      return
    }
    if (data === '\r') {
      script = ''
      return
    }
    if (data === '\b' || data === '\x7f') {
      script = script.slice(0, -1)
      if (script.length == 0) {
        next = [commands]
      }
      return
    }

    script += data
    console.log("script ", script)
    const lastWord = getLastWordsFromScript(script)
    console.log("last word ", lastWord)
    
    for (let i = 0; i < next.length; i++) {
      if (lastWord in next[i]) {
        let current = next[i][lastWord]
        next = current['next']

        console.log("current - ", current)
        console.log("next - ", next)

        break
      }
    }

  }

</script>

<div>
  <div id="terminal"></div>
</div>

<style lang="scss">

</style>
