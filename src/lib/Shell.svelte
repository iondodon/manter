<script lang="ts">

  import { onMount } from "svelte"
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"
  import { Command } from '@tauri-apps/api/shell'

  let script: string = ''
  let history = []
  export let suggestions = []

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

      terminal.open(document.getElementById('terminal'))
      fitAddon.fit()

      terminal.onData(function(data: string) {
        let encodedData = new TextEncoder().encode("\x00" + data)
        websocket.send(encodedData)

        suggestions = getSuggestions(data)
        console.log('suggestions - ', suggestions)
      })

      function updateSuggestionsDivLocation(suggestions) {
        const cursorHtml = document.getElementsByClassName('xterm-helper-textarea')[0]
        const rect = cursorHtml.getBoundingClientRect()
        const suggestionsElement = document.getElementById('suggestions')

        if (script.length == 0) {
          suggestionsElement.style.display = 'none'
          return 
        }

        suggestionsElement.style.display = 'block'
        suggestionsElement.style.top = `${rect.top + 20}px`
        suggestionsElement.style.left = `${rect.left + 10}px`
      }

      terminal.onCursorMove(() => {
        updateSuggestionsDivLocation(suggestions)
      })

      terminal.onResize(function(evt) {
        websocket.send(new TextEncoder().encode("\x01" + JSON.stringify({cols: evt.cols, rows: evt.rows})))
      })

      terminal.buffer.onBufferChange((buf) => {console.log(buf.type)})

      terminal.onTitleChange(function(title) {
        document.title = title
      })

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


  const files = {
      values: function () {
        const script = "this is the script"

        return [
          {
            'names': ['file1.txt'],
            'description': 'description of first file',
            'getNext': function() { return [files] }
          },
          {
            'names': ['file2.txt'],
            'description': 'description of second file',
            'getNext': function() { return [files] }
          }
        ]
      }
  }
  

  const lsOptions = {
    values: [
      {
        'names': ['-a', '--all'],
        'description': 'ls all - description',
        'getNext': function() { return [lsOptions, files] }
      }
    ]
  }

  const commands = {
    values: [
      {
        'names': ['ls'],
        'description': "ls description",
        'getNext': function() { return [lsOptions, files] }
      },
      {
        'names': ['sudo'],
        'description': "super user do",
        'getNext': function() { return [commands] }
      }
    ]
  }

  let next: any = [commands]
  const getSuggestions = (data: string) => {
    console.log("history - ", history)
    console.log("next - ", next)

    if (data === '\n' || data === '\r' || data == '\x03') {
      script = ''
      history = []
      next = [commands]
      return next
    }
    if (data === '\b' || data === '\x7f') {
      script = script.slice(0, -1)
      if (script.length == 0) {
        history = []
        next = [commands]
      }
      if (history[script.length] != undefined) {
        return history[script.length]
      }
      return next
    }

    script += data
    console.log('script - ', script)
    const lastWord = getLastWordsFromScript(script)
    console.log('lastWord - ', lastWord)

    if (script.length == 0) {
      history = []
      next = [commands]
      return next
    }

    for (const candidatesWrapper of next) {
      if (candidatesWrapper['values'] == "function") {
        candidatesWrapper['values'] = candidatesWrapper['values']()
      }

      
      let candidates = candidatesWrapper['values']
      let selected = null
      for (const candidate of candidates) {
        for (const name of candidate['names']) {
          if (name == lastWord) {
            selected = candidate
            break
          }
        }
        if (selected) {
          break
        }
      }

      if (selected) {
        next = selected['getNext']()
        history[script.length] = next

        for (const wrapper of next) {
          if (typeof wrapper['values'] == 'function') {
            wrapper['values'] = wrapper['values']()
          }
        }

        return next
      }
    }

    history[script.length] = next

    return next
  }

</script>

<div>
  <div id="terminal">
    <div id="suggestions">
      {#each suggestions as wrapper}
        <div class="suggestions-wrapper">
          {#each wrapper['values'] as suggestion}
            <div class="suggestion">
              {JSON.stringify(suggestion)}
            </div>
          {/each}
        </div>
      {/each}
    </div>
  </div>
</div>

<style lang="scss">
  #suggestions {
    z-index: 1000;
    position: absolute;
    top: 0;
    left: 0;
    width: 800px;
    height: 200px;
    background-color: rgb(19, 11, 127);
    color: rgb(218, 17, 17);
    font-size: 1.2em;
    font-family: monospace;
    border: 1px solid rgb(222, 21, 21);
    display: none;
  }

  .suggestion {
    border-bottom: 1px solid rgb(26, 12, 12);
  }

  .suggestions-wrapper {
    border-bottom: 1px solid rgb(13, 242, 5);
  }
</style>
