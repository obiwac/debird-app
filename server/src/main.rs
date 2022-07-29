use std::collections::HashMap;
use std::fs::File;
use std::net::SocketAddr;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

use serde_json::{Value, json};

const DB_PATH: &str = "db.json";
const API_VERSION: &str = "1.0";

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
		// respond with current API version

		(&Method::GET, "version") => {
			Ok(Response::new(Body::from(API_VERSION)))
		}

		// respond with information about a given user

		(&Method::GET, "user_info") => {
			if bits.len() != 3 {
				return Ok(not_found(format!("expected 2 args (got {})", bits.len() - 1)));
			}

			let user = String::from(bits[2]);
			println!("Requesting information about user '{}'", user);

			let users: HashMap<String, Value> = serde_json::from_value(db["users"].clone()).unwrap();

			if ! users.contains_key(&user) {
				return Ok(not_found(format!("user {} doesn't exist", user)));
			}

			Ok(Response::new(Body::from(users[&user].to_string())))
		}

		// respond with a sorted list of users, following a given key

		(&Method::GET, "sort") => {
			if bits.len() != 3 {
				return Ok(not_found(format!("expected 2 args (got {})", bits.len() - 1)));
			}

			let field = String::from(bits[2]);
			println!("Requesting sorted list of users by '{}'", field);

			let users: HashMap<String, Value> = serde_json::from_value(db["users"].clone()).unwrap();

			let mut user_list: Vec<(String, usize)> = users.iter().map(|user| {
				let name: String = user.0.as_str().into();
				let events: Vec<Value> = serde_json::from_value(user.1["events"].clone()).unwrap();
				let i = events.iter().filter(|event| event["type"] == field).count();

				(name, i)
			}).collect();

			user_list.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
			Ok(Response::new(Body::from(json!(user_list.clone()).to_string())))
		}

		// respond with error message if the API endpoint doesn't exist

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
