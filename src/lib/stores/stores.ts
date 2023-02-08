import { writable } from "svelte/store";

export const TerminalsStore = writable([]);

export const SessionContextStore = writable({});
export const ActiveTermUUIDStore = writable();
