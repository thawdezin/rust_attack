use std::time::Duration;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tokio::time;

#[tokio::main]
async fn main() {
    let target_url = "https://www..com/";

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    });

    let server = Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(make_svc);

    println!("Slowloris attack started against: {}", target_url);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Introduce a delay to keep the connection open
    time::sleep(Duration::from_secs(30)).await;

    Ok(Response::new(Body::from("This is a slow HTTP response.\n")))
}
