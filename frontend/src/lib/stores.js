import { writable } from "svelte/store";

export const users = writable({})
export const jwt = writable("")
export const invite_jwt = writable(null);