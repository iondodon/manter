import { writable } from "svelte/store";
import { v4 as uuidv4 } from "uuid";

const FIRST_TERM_UUID = uuidv4();

export type TerminalData = {
  uuid: string;
  sessionContext: {
    user: string;
    suggestions: any[];
    scriptResult: any;
    suggestionsContainer: any;
    lineText: string;
    suggestionsSelectedIndex: number;
  };
  terminalInterface: any;
  ptyWebSocket: any;
  fitAddon: any;
};

export const TerminalsStore = writable([
  {
    uuid: FIRST_TERM_UUID,
    sessionContext: {
      user: "",
      suggestions: [],
      scriptResult: {},
      suggestionsContainer: null,
      lineText: "",
      suggestionsSelectedIndex: 0,
    },
    terminalInterface: null,
    ptyWebSocket: null,
    fitAddon: null,
  },
]);

export const SessionContextStore = writable({});
export const ActiveTermUUIDStore = writable(FIRST_TERM_UUID);
