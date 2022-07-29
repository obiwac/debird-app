use std::net::SocketAddr;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

const API_VERSION: &str = "1.0";

async fn respond(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
	let mut not_found = Response::default();
	*not_found.status_mut() = StatusCode::NOT_FOUND;

	let split = req.uri().path().split("/");
	let bits: Vec<&str> = split.collect();

	if bits[0] != "" {
		return Ok(not_found);
	}

	match (req.method(), bits[1]) {
		(&Method::GET, "version") => {
			Ok(Response::new(Body::from(API_VERSION)))
		}

		_ => {
			Ok(not_found)
		}
	}
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let make_service = make_service_fn(|_conn| async {
		Ok::<_, hyper::Error>(service_fn(respond))
	});

	let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
	let server = Server::bind(&addr).serve(make_service);

	println!("Server listening on http://{}", addr);

	server.await?;
	Ok(())
}
