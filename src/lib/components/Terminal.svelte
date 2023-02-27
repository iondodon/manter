<script lang="ts">
  import { Terminal } from 'xterm'
  import { FitAddon } from 'xterm-addon-fit'
  import { CanvasAddon } from 'xterm-addon-canvas'
  import 'xterm/css/xterm.css'
  import { IS_WINDOWS, PTY_WS_ADDRESS } from '../config/config'
  import { arrayBufferToString } from '../utils/utils'
  import { onMount } from 'svelte'
  import { ActiveSessionContextStore } from '../stores/stores'
  import { getSuggestions } from '../../suggestions/suggestions';
  import SuggestionsContainer from './SuggestionsContainer.svelte';
  import clis from '../../cli/library/library';

  export let sessionContext: object
  export let terminalInterface: Terminal
  export let ptyWebSocket: WebSocket
  export let fitAddon: FitAddon

  let suggestionsSelectedIndex = 0

  onMount(() => {
    openDomTerminalInterface()
  })

  const suggestionsAreShown = () => {
    return sessionContext['suggestions'].length > 0 
            && sessionContext['suggestions'][0] !== clis
  }

  const openDomTerminalInterface = () => {
    if (!terminalInterface) {
      return
    }
    terminalInterface.open(document.getElementById('terminal'))
    sendProposedSizeToPty()
    adjustDomTerminalElementSize()
    terminalInterface.focus()
  }

  const sendProposedSizeToPty = () => {
    const proposedSize = fitAddon.proposeDimensions()
    const resizeData = {
      cols: proposedSize.cols,
      rows: proposedSize.rows,
      pixel_width: 0,
      pixel_height: 0
    }
    ptyWebSocket.send(
      new TextEncoder().encode('\x01' + JSON.stringify(resizeData))
    )
  }

  const adjustDomTerminalElementSize = () => {
    fitAddon.fit()

    const terminal = document.getElementById('terminal') as HTMLElement
    const terminalHeight = terminal.clientHeight
    const terminalWidth = terminal.clientWidth

    const xtermElement = document.getElementsByClassName('xterm')[0] as HTMLElement
    const xtermViewPortElement = document.getElementsByClassName('xterm-viewport')[0] as HTMLElement

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
    ptyWebSocket.send(
      new TextEncoder().encode('\x01' + JSON.stringify(resizeValues))
    )
    adjustDomTerminalElementSize()
  }

  const termInterfaceHandleUserInputData = (data: string) => {
    const encodedData = new TextEncoder().encode('\x00' + data)
    ptyWebSocket.send(encodedData)
  }

  const termInterfaceHandleTitleChange = (title) => {
    if (title.includes('[manter]')) {
      title = title.replace('[manter]', '')
      let promptUpdatedData = {}
      try {
        promptUpdatedData = JSON.parse(title)
      } catch (e) {
        alert('Error parsing JSON from prompt_command_script\n' + e.message)
        return
      }
      sessionContext['prompt_command_result'] = promptUpdatedData
      ActiveSessionContextStore.update(() => sessionContext)
      return
    }
    document.title = title
  }

  const getTypedText = () => {
    // @ts-ignore
    const scrolledRows = terminalInterface.buffer.active._buffer.ydisp

    let lineIndex = terminalInterface.buffer.active.cursorY + scrolledRows
    let line = terminalInterface.buffer.active.getLine(lineIndex)
    let text = line.translateToString(true, 0, terminalInterface.buffer.active.cursorX)

    while (line.isWrapped) {
      line = terminalInterface.buffer.active.getLine(--lineIndex)
      const currentLineText = line.translateToString(true)
      text = currentLineText + text
    }

    return text
  }

  const getSuggestionByIndex = (index) => {
    for(const suggestion of sessionContext['filteredSuggestions']) {
      if (suggestion.suggestions) {
        for (const subSuggestion of suggestion.suggestions) {
          if (subSuggestion.index == index) {
            return subSuggestion
          }
        }
      } else {
        if (suggestion.index == index) {
          return suggestion
        }
      }
    }

    console.log('Suggestion not found for index ' + index)
    return null
  }

  const getLastWord = () => {
    return sessionContext['lineText']
      .split(' ')
      .map((word) => word.trim())
      .filter((word) => word.length > 0)
      .pop()
  }

  const atLeastOneNameStartsWith = (names: string | string[], word) => {
    if (typeof names === 'string') {
      names = [names]
    }

    for (const name of names) {
      if (name.startsWith(word)) {
        return true
      }
    }

    return false
  }

  const filterSuggestions = (sessionContext) => {
    const suggestions = sessionContext['suggestions']
    const lastWord = getLastWord()

    return suggestions.map((suggestion) => {
      if (suggestion.suggestions) {
        const filteredSubSuggestions = suggestion.suggestions.filter((subSuggestion) => {
          return atLeastOneNameStartsWith(subSuggestion.names, lastWord)
        })

        return {
          ...suggestion,
          suggestions: filteredSubSuggestions
        }
      } else {
        return suggestion
      }
    }).filter((suggestion) => {
      if (suggestion.suggestions) {
        return suggestion.suggestions.length > 0
      } else {
        return atLeastOneNameStartsWith(suggestion.names, lastWord)
      }
    })
  }
  
  const termInterfaceHandleKeyEvents = (evt) => {
    if (suggestionsAreShown() && evt.key === 'ArrowUp') {
      return false
    }

    if (suggestionsAreShown() && evt.key === 'ArrowDown') {
      return false
    }

    if (suggestionsAreShown() && evt.key === 'Enter') {

      if (evt.type == 'keyup') {
        const selectedSuggestion = getSuggestionByIndex(suggestionsSelectedIndex)

        if (selectedSuggestion) {
          const lastWord = getLastWord()

          if (typeof selectedSuggestion.names === 'string') {
            selectedSuggestion.names = [selectedSuggestion.names]
          }
          let foundMatch = false
          for (const name of selectedSuggestion.names) {
            if (name.startsWith(lastWord)) {
              terminalInterface.paste(name.replace(lastWord, ''))
              foundMatch = true
              break
            }
          }

          if (!foundMatch) {
            terminalInterface.paste(selectedSuggestion.names[0])
          }
        }

        sessionContext['suggestions'] = []
        sessionContext['filteredSuggestions'] = []
        suggestionsSelectedIndex = 0
      }
      
      return false
    }

    if (evt.ctrlKey && evt.key === 'v') {
      if (evt.type == 'keydown') {
        console.log('Pasting text')
      }
      return false
    }

    if (evt.ctrlKey && evt.key === 'c') {
      if (evt.type == 'keydown') {
        if (terminalInterface.hasSelection()) {
          terminalInterface.clearSelection()
          return false
        }
      }
    }

    if (evt.key === 'Escape') {
      if (sessionContext['filteredSuggestions'].length > 0) {
        sessionContext['suggestions'] = []
        sessionContext['filteredSuggestions'] = []
        console.log('Escape key pressed - suggestions cleared')
        return false
      }
      if (evt.type == 'keydown') {
        console.log('Escape key pressed')
        return true
      }
      console.log('Escape key released')
      return false
    }

    if (evt.type === 'keyup' && evt.ctrlKey && evt.key === '=') {
      terminalInterface.options.fontSize += 1
      sendProposedSizeToPty()
      fitAddon.fit()
      return false
    }

    if (evt.type === 'keyup' && evt.ctrlKey && evt.key === '-') {
      terminalInterface.options.fontSize -= 1
      sendProposedSizeToPty()
      fitAddon.fit()
      return false
    }

    if (evt.type === 'keyup') {
      const lineText = getTypedText()
      sessionContext['lineText'] = lineText
      
      if (lineText.endsWith(' ')) {
        const suggestions = getSuggestions(sessionContext)
        sessionContext['suggestions'] = suggestions
        sessionContext['filteredSuggestions'] = suggestions
      } else {
        sessionContext['filteredSuggestions'] = filterSuggestions(sessionContext)
      }

      suggestionsSelectedIndex = 0
      ActiveSessionContextStore.update((_prevSessionContext) => sessionContext)
    }

    return true
  }

  const termInterfaceHandleCursorMove = () => {
    
  }

  const writePtyIncomingToTermInterface = (evt) => {
    if (!(evt.data instanceof ArrayBuffer)) {
      alert('unknown data type ' + evt.data)
      return
    }
    const dataString: string = arrayBufferToString(evt.data.slice(1))
    terminalInterface.write(dataString)
    return dataString
  }

  const handlePtyWsClose = (_evt) => {
    terminalInterface.write('Terminal session terminated')
    terminalInterface.dispose()
    console.log('websocket closes from backend side')
  }

  const handlePtyWsError = (evt) => {
    if (typeof console.log == 'function') {
      console.log('ws error', evt)
    }
  }

  const setupNewTerminalInterface = () => {
    terminalInterface = new Terminal({
      cursorBlink: true,
      cursorStyle: 'block',
      fontSize: 16,
      cursorWidth: 8,
    })
    fitAddon = new FitAddon()
    terminalInterface.loadAddon(fitAddon)

    const canvasAddon = new CanvasAddon()
    terminalInterface.loadAddon(canvasAddon)

    terminalInterface.attachCustomKeyEventHandler((evt) => termInterfaceHandleKeyEvents(evt))
    terminalInterface.onKey((evt) => {})

    openDomTerminalInterface()

    addEventListener('resize', (_event) => adjustDomTerminalElementSize())
    terminalInterface.onResize((evt) => termInterfaceHandleResize(evt))
    terminalInterface.onData((data) => termInterfaceHandleUserInputData(data))
    terminalInterface.onCursorMove(() => termInterfaceHandleCursorMove())
    terminalInterface.buffer.onBufferChange((_buff) => {})
    terminalInterface.onTitleChange((title) =>termInterfaceHandleTitleChange(title))

    if (IS_WINDOWS) {
      terminalInterface.options.windowsMode = true
    }
  }

  const newTerminalSession = async () => {
    ptyWebSocket = new WebSocket(PTY_WS_ADDRESS)
    ptyWebSocket.binaryType = 'arraybuffer'
    ptyWebSocket.onmessage = writePtyIncomingToTermInterface
    ptyWebSocket.onclose = (evt) => handlePtyWsClose(evt)
    ptyWebSocket.onerror = (evt) => handlePtyWsError(evt)
    ptyWebSocket.onopen = async (_evt) => setupNewTerminalInterface()
  }

  if (ptyWebSocket == null) {
    newTerminalSession()
  }
</script>

<div id="terminal" />
<SuggestionsContainer sessionContext={sessionContext} bind:selectedIndex={suggestionsSelectedIndex} />

<style lang="scss">
  #terminal {
    width: 100%;
    height: calc(100vh - 80px);
    background-color: black;
    overflow: hidden;
  }
</style>
