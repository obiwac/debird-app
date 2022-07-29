use std::collections::HashMap;
use std::fs::File;
use std::net::SocketAddr;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

use serde::Deserialize;
use serde_json::{Value, json};

const DB_PATH: &str = "db.json";
const API_VERSION: &str = "1.0";

#[derive(Deserialize, Debug)]
struct DBUser {
	klapgijp: String
}

fn not_found(msg: String) -> Response<Body> {
	Response::builder()
		.status(StatusCode::NOT_FOUND)
		.body(format!("404 Not found: {}", msg).into())
		.unwrap()
}

async fn respond(req: Request<Body>, db: Value) -> Result<Response<Body>, hyper::Error> {
	let split = req.uri().path().split("/");
	let bits: Vec<&str> = split.collect();

	if bits[0] != "" {
		return Ok(not_found("invalid HTTP GET request format".into()));
	}

	match (req.method(), bits[1]) {
		(&Method::GET, "version") => {
			Ok(Response::new(Body::from(API_VERSION)))
		}

		(&Method::GET, "user_info") => {
			let user = String::from(bits[2]);
			println!("Requesting information about user '{}'", user);

			// XXX this is a very temporary solution

			let users: HashMap<String, Value> = serde_json::from_str(&db["users"].to_string()).unwrap();

			if ! users.contains_key(&user) {
				return Ok(not_found(format!("user {} doesn't exist", user)));
			}

			Ok(Response::new(Body::from(users[&user].to_string())))
		}

		_ => {
			Ok(not_found(format!("unknown API endpoint: {}", bits[1])))
		}
	}
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	// read database
	// if it doesn't yet exist, create it

	let db: Value = match File::open(DB_PATH) {
		Ok(file) => {
			serde_json::from_reader(file)
				.expect("file should be JSON")
		}
		Err(err) => {
			eprintln!("Failed to open {} for reading: {}", DB_PATH, err);
			json!({})
		}
	};

	// create server

	let make_service = make_service_fn(move |_conn| {
		let db = db.clone();
		async move { Ok::<_, hyper::Error>(service_fn(move |req| respond(req, db.clone()))) }
	});

	let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
	let server = Server::bind(&addr).serve(make_service);

	println!("Server listening on http://{}", addr);

	server.await?;
	Ok(())
}
