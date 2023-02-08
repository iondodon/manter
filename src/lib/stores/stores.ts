import { writable } from "svelte/store";

export const TerminalsStore = writable([]);

export const ActiveSessionContextStore = writable({});
export const ActiveTermUUIDStore = writable();
