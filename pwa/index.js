// API interaction functions

const SERVER_ADDR = "127.0.0.1"
const SERVER_PORT = 6969

function api_get(cmd, param, cb) {
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

function api_post(cmd, body) {
	var http = new XMLHttpRequest()

	http.onreadystatechange = function() {
		if (http.readyState !== 4) {
			return
		}

		if (http.status !== 200) {
			return
		}

		console.log("response for", cmd)
	}

	const uri = `http://${SERVER_ADDR}:${SERVER_PORT}/${cmd.join('/')}`

	http.open("POST", uri, true /* async */)
	http.send(JSON.stringify(body))
}

// higher-level functionality

function create_account(name) {
	// generate keypair specific to user

	const keypair = SubtleCrypto.generateKey({
		name: "RSA-OAEP",
		modulusLength: 4096,
		publicExponent: new Uint8Array([1, 0, 1]),
		hash: "SHA-256"
	}, true, ["encrypt", "decrypt"])

	api_post(["create_account"], {
		"test": "value"
	})
}

// testing

api_get(["version"], [], function(version) {
	document.getElementById("api_version").innerText = "API version: " + version
})

api_get(["sort", "klapgijp"], [], function(list) {
	list = JSON.parse(list)
	let str = ""

	for (const ent of list) {
		str += `${ent[0]}\t${ent[1]} klapgijpen\n`
	}

	document.getElementById("klapgijp_board").innerText = str
})
