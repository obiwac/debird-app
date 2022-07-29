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
		(&Method::GET, "version") => {
			Ok(Response::new(Body::from(API_VERSION)))
		}

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

		(&Method::GET, "sort") => {
			if bits.len() != 3 {
				return Ok(not_found(format!("expected 2 args (got {})", bits.len() - 1)));
			}

			let field = String::from(bits[2]);
			println!("Requesting sorted list of users by '{}'", field);

			let users: HashMap<String, Value> = serde_json::from_value(db["users"].clone()).unwrap();
			let mut user_list: Vec<_> = users.iter().collect();

			user_list.sort_by(|a, b| {
				let i: u64 = a.1[&field].as_u64().unwrap();
				let j: u64 = b.1[&field].as_u64().unwrap();

				j.partial_cmp(&i).unwrap()
			});

			let mut response: Vec<(String, u64)> = vec![];

			for user in user_list {
				let name: String = user.0.as_str().into();
				let i: u64 = user.1[&field].as_u64().unwrap();

				response.push((name, i));
			}

			Ok(Response::new(Body::from(json!(response).to_string())))
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