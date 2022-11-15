export async function fetchUser(id) {
	let user = await fetch(`/api/users/info/${id}`);
	user = await user.json();
	return user;
}
