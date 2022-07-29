use std::fs::File;
use std::net::SocketAddr;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

use serde_json::{Value};

const DB_PATH: &str = "db.json";
const API_VERSION: &str = "1.0";

fn not_found() -> Response<Body> {
	Response::builder()
		.status(StatusCode::NOT_FOUND)
		.body("404 Not found".into())
		.unwrap()
}

async fn respond(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
	let split = req.uri().path().split("/");
	let bits: Vec<&str> = split.collect();

	if bits[0] != "" {
		return Ok(not_found());
	}

	match (req.method(), bits[1]) {
		(&Method::GET, "version") => {
			Ok(Response::new(Body::from(API_VERSION)))
		}

		(&Method::GET, "users") => {
			let sort = String::from(bits[2]);
			println!("{}", sort);

			Ok(Response::new(Body::from(sort)))
		}

		_ => {
			Ok(not_found())
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
			serde_json::from_str("{}")?
		}
	};

	println!("{}", db);

	// create server

	let make_service = make_service_fn(|_conn| async {
		Ok::<_, hyper::Error>(service_fn(respond))
	});

	let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
	let server = Server::bind(&addr).serve(make_service);

	println!("Server listening on http://{}", addr);

	server.await?;
	Ok(())
}
