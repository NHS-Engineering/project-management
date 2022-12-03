import { jwt, jwt_claims } from "./stores.js";
import { get } from "svelte/store";

export async function login(username, password) {
	let resp = await fetch("/api/auth/login", {
		"method": "POST",
		"body": JSON.stringify({
			"username": username,
			"password": password
		})
	});

	if (!resp.ok) {
		throw "failed to login";
	}

	let jwt = await resp.text();

	setJwt(jwt);
}

function setJwt(new_jwt) {
	jwt.set(new_jwt);

	const now = new Date().getTime();
	const delta_expires = (get(jwt_claims)["exp"] * 1000) - now;

	setTimeout(() => {
		jwt.set("");
		console.debug("session expired");
	}, delta_expires);
}