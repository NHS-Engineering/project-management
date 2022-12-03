export function get_jwt_claims(jwt) {
	return JSON.parse(atob(jwt.split(".")[1]));
}