<script lang="ts">

  import { onMount } from "svelte"
  import { Terminal }  from "xterm"
  import { FitAddon }   from "xterm-addon-fit"
  import "xterm/css/xterm.css"
  import { spawn } from "./command"

  export let script: string = '' 
  export let lastWord = ''
  export let suggestions = []
  let history = []
  export let promptContext = {
    cwd: "~"
  }

  export const isVisible = (suggestion) => {
    if (script[script.length - 1] == " ") {
      return true
    }
    for (const name of suggestion["names"]) {
      if (name == lastWord || name.startsWith(lastWord)) {
        return true
      }
    }
    return false
  }

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

      terminal.onData(async function(data: string) {
        let encodedData = new TextEncoder().encode("\x00" + data)
        websocket.send(encodedData)

        await getSuggestions(data)
        suggestions = history[script.length]
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

      terminal.onSelectionChange(() => {
        websocket.send(new TextEncoder().encode("\x03"))
      })

      terminal.buffer.onBufferChange((buf) => {console.log(buf.type)})

      terminal.onTitleChange(function(title) {
        if (title.includes("[manter]")) {
            title = title.replace("[manter]", "")
            promptContext = JSON.parse(title)
            return
        }
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
      script: "ls -a",
      processor: function (line) {
        return {
          'names': [line],
          'description': 'description of first file',
          'getNext': function() { return [files] }
        }
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

  history = [ [commands] ]
  const getSuggestions = async (data: string) => {
    if (data === '\n' || data === '\r' || data == '\x03') {
      history = [ [commands] ]
      script = ''
      return
    }

    if (data === '\b' || data === '\x7f') {
      if (script.length > 0) {
        script = script.slice(0, -1)
      }
    } else {
      script += data
    }

    lastWord = getLastWordsFromScript(script)

    if (script.length == 0) {
      return
    }

    let selected = null
    for (let candidatesWrapper of history[script.length - 1]) {
      if (candidatesWrapper['processor']) {
        candidatesWrapper['values'] = await spawn(candidatesWrapper, promptContext["cwd"])
        console.log("finished receiving data top", candidatesWrapper['values'])
      }
      
      let candidates = candidatesWrapper['values']
      for (const candidate of candidates) {
        for (const name of candidate['names']) {
          if (!selected && name == lastWord) {
            selected = candidate
            break
          }
        }
        if (selected) {
          break
        }
      }

    }

    if (!selected) {
      history[script.length] = history[script.length - 1]
      return
    }

    history[script.length] = selected['getNext']()
    for (let wrapper of history[script.length]) {
      if (wrapper['processor']) {
        wrapper['values'] = await spawn(wrapper, promptContext["cwd"])
        console.log("finished receiving data bottom", wrapper['values'])
      }
    }

  }

</script>

<div>
  <div id="terminal">
    <div id="suggestions">
      
      {#if suggestions}
        {#each suggestions as wrapper}
          <div class="suggestions-wrapper suggestion-item">
            {#if wrapper["values"]}
              {#each wrapper['values'] as suggestion}
                {#if isVisible(suggestion)}
                  <div class="suggestion suggestion-item">
                      {JSON.stringify(suggestion["names"])}
                  </div>
                {/if}
              {/each}
            {/if}
          </div>
        {/each}
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
  #suggestions {
    z-index: 1000;
    position: absolute;
    top: 0;
    left: 0;
    width: fit-content;
    max-height: 200px;
    background-color: rgb(19, 11, 127);
    color: rgb(218, 17, 17);
    font-size: 1.2em;
    font-family: monospace;
    border: 1px solid rgb(222, 21, 21);
    display: none;
    overflow-y: scroll;
  }

  .suggestion {
    border-bottom: 1px solid rgb(0, 0, 0);
  }

  .suggestions-wrapper {
    border-bottom: 3px solid rgb(7, 115, 3);
  }

  .suggestion-item {
    margin: 2px;
  }
</style>
