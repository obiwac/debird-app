const SERVER_ADDR = "127.0.0.1"
const SERVER_PORT = 6969

function api_request(cmd, param, cb) {
	var http = new XMLHttpRequest()

	http.onreadystatechange = function() {
		if (http.readyState !== 4) {
			return
		}

		if (http.status !== 200) {
			return
		}

		cb(http.responseText, ...param)
	}

	const uri = `http://${SERVER_ADDR}:${SERVER_PORT}/${cmd.join('/')}`

	http.open("GET", uri, true /* async */)
	http.send(null)
}

api_request(["version"], [], function(version) {
	document.getElementById("api_version").innerText = version
})
