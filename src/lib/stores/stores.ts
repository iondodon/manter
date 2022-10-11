import { writable } from 'svelte/store'

export const TerminalsStore = writable([
  {
    id: 0, 
    sessionContext: {
      isLoggedIn: false,
      cwd: "~",
      user: ""
    },
    terminalInterface: null,
    ptyWebSocket: null,
    fitAddon: null
  }
])

export const ActiveTermIdStore = writable(0)