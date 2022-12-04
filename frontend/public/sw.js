const cache_name = "cache";

self.addEventListener("install", function(event) {
	console.log("serviceworker registered");

	caches.open(cache_name).then(function(cache) {
		return cache.addAll(["/"]); // requires reload to be fully cached, but works mostly well enough
	});
});

// network first then defer to cache unless non-GET or otherwise specified
self.addEventListener("fetch", function(event) {
	event.respondWith(fetch(event.request).then(function(response) {
		if (event.request.method === "GET") {
			caches.open(cache_name).then(function(cache) {
				cache.add(event.request);
			});
		}

		if (response.ok) {
			sendOnlineStatus(event.clientId, true);
		}

		return response;
	}).catch(function() {
		sendOnlineStatus(event.clientId, false);

		if (event.request.headers.get("Cache-Control") !== "no-cache") {
			return caches.match(event.request).then(function(response) {
				return response;
			});
		}
	}));
});

function sendOnlineStatus(clientId, isOnline) {
	if (!clientId) return;

	clients.get(clientId).then(client => {
		if (!client) return;
		client.postMessage({"onlineStatus": isOnline});
	});
}
