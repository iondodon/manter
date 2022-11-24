<script lang="ts">
  import { Terminal } from 'xterm'
  import { FitAddon } from 'xterm-addon-fit'
  import 'xterm/css/xterm.css'
  import { IS_UNIX, PTY_WS_ADDRESS } from '../config/config'
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
    adjustTerminalSize()
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

  const adjustTerminalSize = () => {
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
    adjustTerminalSize()
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
    terminalInterface.write(BANNER)
    terminalInterface.write("    v. 0.0.1 \r\n\n")
    if (IS_UNIX) {
      terminalInterface.write("User: " + sessionContext['user'] + "\r\n")
    }
  }

  const setupNewTerminalInterface = () => {
    terminalInterface = new Terminal({
      cursorBlink: true,
      cursorStyle: 'bar',
      cursorWidth: 6
    })
    fitAddon = new FitAddon()
    terminalInterface.loadAddon(fitAddon)

    openDomTerminalInterface()

    addEventListener('resize', (_event) => adjustTerminalSize())
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
    terminalInterface.write('Session terminated')
    terminalInterface.dispose()
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
    align-items: center;

    top: 0em;
    right: 0em;
    height: calc(100% - 1.2em);
    width: 100%;

    background-color: black;
    overflow: hidden;
  }
</style>
