import { jwt } from "./stores.js";

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
	const parts = new_jwt.split(".");
	const claims = JSON.parse(atob(parts[1]));

	const now = new Date().getTime();
	const delta_expires = (claims["exp"] * 1000) - now;

	setTimeout(() => {
		jwt.set("");
		console.debug("session expired");
	}, delta_expires);

	jwt.set(new_jwt);
}