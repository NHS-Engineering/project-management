import { writable } from "svelte/store";
import { get_jwt_claims } from "./jwt.js";

export const users = writable({});
export const jwt = writable("");
export let jwt_claims = writable({});
jwt.subscribe(jwt => {
	try {
		jwt_claims.set(get_jwt_claims(jwt));
	} catch {
		jwt_claims.set({});
	}
});

export const invite_jwt = writable(null);

export const messages = writable([]);