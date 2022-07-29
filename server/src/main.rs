use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn responder(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
	Ok(Response::new(Body::from("Test response")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let make_service = make_service_fn(|_conn| async {
		Ok::<_, Infallible>(service_fn(responder))
	});

	let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
	let server = Server::bind(&addr).serve(make_service);

	println!("Server listening on http://{}", addr);

	server.await?;
	Ok(())
}
