<script lang="ts">
  import { Terminal } from 'xterm'
  import { FitAddon } from 'xterm-addon-fit'
  import 'xterm/css/xterm.css'
  import { IS_MACINTOSH, IS_UNIX, IS_WINDOWS, PTY_WS_ADDRESS } from '../config/config'
  import { arrayBufferToString } from '../utils/utils'
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  import { SessionContextStore } from '../stores/stores'
  import { BANNER } from '../../banner'
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

    if (IS_UNIX) {
      setUser()
    }
  })

  const suggestionsAreShown = () => {
    return sessionContext['suggestions'].length > 0 
            && sessionContext['suggestions'][0] !== clis 
            && sessionContext['lineText'].endsWith(' ')
  }

  const setUser = async () => {
    const settings = (await invoke('get_settings')) as string
    sessionContext['user'] = settings['default_login_user']
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
      sessionContext['scriptResult'] = promptUpdatedData
      SessionContextStore.update(() => sessionContext)
      return
    }
    document.title = title
  }

  const displayWelcomePage = () => {
    if (IS_UNIX) {
      terminalInterface.write(BANNER)
      terminalInterface.write("    v. 0.1.0 \r\n")
      terminalInterface.write("    Licensed under GPL v.3 \r\n\n")

      if (IS_MACINTOSH) {
        terminalInterface.write("Run these two commands in the terminal after login: \r\n\n")
        terminalInterface.write("$ prmptcmd() { eval \"$PROMPT_COMMAND\" }  \r\n")
        terminalInterface.write("$ precmd_functions=(prmptcmd)  \r\n\n")
        terminalInterface.write("They are needed to run the script that shows at the bottom information such as the current directory, the git branch, the user, etc. \r\n\n")
      }

      terminalInterface.write("User: " + sessionContext['user'] + "\r\n")
    }
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
  
  const termInterfaceHandleKeyEvents = (evt) => {
    if (suggestionsAreShown() && evt.key === 'ArrowUp') {
      return false
    }

    if (suggestionsAreShown() && evt.key === 'ArrowDown') {
      return false
    }

    if (suggestionsAreShown() && evt.key === 'Enter') {
      return false
    }

    if (suggestionsAreShown() && evt.key === 'Escape') {
      return false
    }

    if (evt.ctrlKey && evt.key === '=') {
      terminalInterface.options.fontSize += 1
      sendProposedSizeToPty()
      fitAddon.fit()
      return false
    }

    if (evt.ctrlKey && evt.key === '-') {
      terminalInterface.options.fontSize -= 1
      sendProposedSizeToPty()
      fitAddon.fit()
      return false
    }

    if (evt.type === 'keyup') {
      const lineText = getTypedText()
      sessionContext['lineText'] = lineText
      const suggestions = getSuggestions(sessionContext)
      sessionContext['suggestions'] = suggestions
      suggestionsSelectedIndex = 0
      SessionContextStore.update((_prevSessionContext) => sessionContext)
    }

    return true
  }

  const termInterfaceHandleCursorMove = () => {
    
  }

  const setupNewTerminalInterface = () => {
    terminalInterface = new Terminal({
      cursorBlink: true,
      cursorStyle: 'bar',
      fontSize: 16,
      cursorWidth: 8,
    })
    fitAddon = new FitAddon()
    terminalInterface.loadAddon(fitAddon)

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

    displayWelcomePage()
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

  const newTerminalSession = async () => {
    ptyWebSocket = new WebSocket(PTY_WS_ADDRESS)
    ptyWebSocket.binaryType = 'arraybuffer'

    ptyWebSocket.onopen = async (_evt) => {
      setupNewTerminalInterface()
      ptyWebSocket.onmessage = writePtyIncomingToTermInterface
      ptyWebSocket.onclose = (evt) => handlePtyWsClose(evt)
      ptyWebSocket.onerror = (evt) => handlePtyWsError(evt)
    }
  }

  if (ptyWebSocket == null) {
    newTerminalSession()
  }
</script>

<div id="terminal" />
<SuggestionsContainer
  bind:this={sessionContext['suggestionsContainer']} 
  suggestions={sessionContext['suggestions']}
  sessionContext={sessionContext}
  bind:selectedIndex={suggestionsSelectedIndex}
/>

<style lang="scss">
  #terminal {
    width: 100%;

    height: calc(100vh - 80px);

    background-color: black;
    overflow: hidden;
  }
</style>
