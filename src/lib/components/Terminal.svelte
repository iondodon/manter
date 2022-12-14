<script lang="ts">
  import { Terminal } from 'xterm'
  import { FitAddon } from 'xterm-addon-fit'
  import 'xterm/css/xterm.css'
  import { IS_UNIX, IS_WINDOWS, PTY_WS_ADDRESS } from '../config/config'
  import { arrayBufferToString } from '../utils/utils'
  import { invoke } from '@tauri-apps/api/tauri'
  import { onMount } from 'svelte'
  import { SessionContextStore } from '../stores/stores'
  import { BANNER } from '../../banner'

  export let sessionContext: any
  export let terminalInterface: Terminal
  export let ptyWebSocket: WebSocket
  export let fitAddon: FitAddon

  onMount(() => {
    openDomTerminalInterface()
    if (IS_UNIX) {
      setUser()
    }
  })

  const setUser = async () => {
    const settingsString = (await invoke('get_settings')) as string
    const settings = JSON.parse(settingsString)
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

    const xtermElement = document.getElementsByClassName(
      'xterm'
    )[0] as HTMLElement
    const xtermViewPortElement = document.getElementsByClassName(
      'xterm-viewport'
    )[0] as HTMLElement

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
      let promptUpdatedData = JSON.parse(title)
      sessionContext = { ...sessionContext, ...promptUpdatedData }
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
      terminalInterface.write("User: " + sessionContext['user'] + "\r\n")
    }
  }
  
  const termInterfaceHandleKeyEvents = (evt) => {
    console.log("xterm intercepted key", evt)

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
    
    return true
  }

  const setupNewTerminalInterface = () => {
    terminalInterface = new Terminal({
      cursorBlink: true,
      cursorStyle: 'bar',
      cursorWidth: 6
    })
    fitAddon = new FitAddon()
    terminalInterface.loadAddon(fitAddon)

    terminalInterface.attachCustomKeyEventHandler((evt) => termInterfaceHandleKeyEvents(evt))

    openDomTerminalInterface()

    addEventListener('resize', (_event) => adjustDomTerminalElementSize())
    terminalInterface.onResize((evt) => termInterfaceHandleResize(evt))
    terminalInterface.onData((data) => termInterfaceHandleUserInputData(data))
    terminalInterface.onCursorMove(() => {})
    terminalInterface.buffer.onBufferChange((_buff) => {})
    terminalInterface.onTitleChange((title) =>
      termInterfaceHandleTitleChange(title)
    )

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
    console.log('Terminal session terminated')
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

<style lang="scss">
  #terminal {
    display: flex;
    justify-content: center;
    align-items:flex-start;

    top: 0em;
    right: 0em;
    height: calc(100% - 4em);
    width: 100%;

    background-color: black;
    overflow: hidden;
  }
</style>
