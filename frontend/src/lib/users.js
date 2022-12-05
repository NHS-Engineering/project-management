import { users } from "./stores.js";
import { get } from "svelte/store";

export async function fetchUser(id) {
	if (id === undefined) {
		return null;
	}

	let cached = get(users)[id];
	if (cached !== undefined) {
		return cached;
	} else {
		users.update(users => {
			users[id] = fetch(`/api/users/info/${id}`)
				.then(resp => resp.json());
			return users;
		});
	}

	try {
		return get(users)[id];
	} catch {
		return null;
	}
}
