import { jwt, messages } from "./stores.js";
import { get_jwt_claims } from "./jwt.js";

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

	resp = await resp.json();
	if (resp["weak_hint"] !== "Valid") {
		const reason = password_reason(resp["weak_hint"]);
		messages.update(messages => {
			messages.push("WARNING: password does not meet strength requirements, change it ASAP:\n" + reason);
			return messages;
		})
	}
	let jwt = resp["jwt"];

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
	const jwt_claims = get_jwt_claims(new_jwt);

	const now = Date.now();
	const delta_expires = (jwt_claims["exp"] * 1000) - now;

	const delta_at = (jwt_claims["iat"] * 1000) - now;
	if (Math.abs(delta_at) > 10 * 1000) {
		alert("your computer's clock is wrong, you may experience problems with this site");
	}

	setTimeout(() => {
		logout();
		console.debug("session expired");
	}, delta_expires);

	jwt.set(new_jwt);
}

export function logout() {
	messages.set([]);
	jwt.set("");
}

export async function manualLogout() {
	if ("PasswordCredential" in window) {
		await navigator.credentials.preventSilentAccess();
	}

	logout();
}

export function tryAutoLogin(silent) {
	if ("PasswordCredential" in window) {
		navigator.credentials.get({
			"password": true,
			"mediation": silent ? "silent" : "optional"
		}).then(creds => {
			if (creds !== null) {
				login(creds.id, creds.password).catch(() => {
					alert("failed to log you in automatically, manually log in with correct password to fix");
				});
			}
		});
	}
}

export function password_reason(reason) {
	if (reason === "Valid") {
		return "looks pretty valid to me...";
	}

	if ("BasicRequirement" in reason) {
		return reason["BasicRequirement"]
	}

	if ("BannedWord" in reason) {
		return `your password contains a banned word: "${reason["BannedWord"]}"`
	}

	alert("unknown invalid password reason");
	console.log(reason);
	return null;
}