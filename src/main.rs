use std::sync::Arc;
use std::time::Duration;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use tokio::time;
use tokio::sync::Mutex;

async fn service_handler(req: Request<Body>, counter: Arc<Mutex<usize>>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let mut interval = time::interval(Duration::from_millis(10));
            let mut counter = counter.lock().await;
            interval.tick().await;
            interval.tick().await;
            interval.tick().await;
            interval.tick().await;
            *counter += 1;
            Ok(Response::default())
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();
    let counter = Arc::new(Mutex::new(0usize));
    let service = make_service_fn(|_| {
        let counter = counter.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |request: Request<Body>| service_handler(request, counter.clone())))
        }
    });

    let server = Server::bind(&addr)
        .serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
