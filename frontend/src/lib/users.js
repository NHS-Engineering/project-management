import { users } from "./stores.js";
import { get } from "svelte/store";

export async function fetchUser(id) {
	if (id === undefined) {
		return null;
	}

	let maybe_cached = get(users)[id];
	if (maybe_cached !== undefined) {
		return maybe_cached;
	}

	try {
		let user = await fetch(`/api/users/info/${id}`);
		user = await user.json();
		users.update(users => {
			users[id] = user;
			return users;
		});
		return user;
	} catch {
		return null;
	}
}
