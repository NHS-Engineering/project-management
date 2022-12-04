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

	if ("PasswordCredential" in window) {
		const cred = new PasswordCredential({
			id: username,
			password: password
		});

		await navigator.credentials.store(cred);
	}

	setJwt(jwt);
}

function setJwt(new_jwt) {
	jwt.set(new_jwt);

	const now = Date.new();
	const delta_expires = (get(jwt_claims)["exp"] * 1000) - now;

	const delta_at = (get(jwt_claims)["iat"] * 1000) - now;
	if (Math.abs(delta_at) > 10 * 1000) {
		alert("your computer's clock is wrong, you may experience problems with this site");
	}

	setTimeout(() => {
		logout();
		console.debug("session expired");
	}, delta_expires);
}

export function logout() {
	jwt.set("");
}

export async function manualLogout() {
	if ("PasswordCredential" in window) {
		await navigator.credentials.preventSilentAccess();
	}

	logout();
}
