const cache_name = "cache";

self.addEventListener("install", function(event) {
	console.log("serviceworker registered");

	caches.open(cache_name).then(function(cache) {
		return cache.addAll(["/"]); // requires reload to be fully offline, but works mostly well enough
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
		return response;
	}).catch(function() {
		if (event.request.headers.get("Cache-Control") !== "no-cache") {
			return caches.match(event.request).then(function(response) {
				return response;
			});
		}
	}));
});
