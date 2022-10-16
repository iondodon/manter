import { writable } from 'svelte/store'
import { v4 as uuidv4 } from 'uuid'

const FIRST_TERM_UUID = uuidv4()

export const TerminalsStore = writable([
  {
    uuid: FIRST_TERM_UUID, 
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

export const ActiveTermUUIDStore = writable(FIRST_TERM_UUID)