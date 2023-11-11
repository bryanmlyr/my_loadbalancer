mod loadbalancer;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;

use hyper_tls::HttpsConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Error};
use tokio::sync::Mutex;
use hyper::Client;
use hyper::client::HttpConnector;
use crate::loadbalancer::LoadBalancer;

async fn router(req: Request<Body>, client: Arc<Client<HttpsConnector<HttpConnector>>>, load_balancer: Arc<Mutex<LoadBalancer>>) -> Result<Response<Body>, Error> {
    load_balancer.lock().await.redirect(client.deref(),req.uri().to_string().as_str()).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let servers_from_env: Vec<String> = env!("LB_SERVERS")
        .to_string()
        .split(",")
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    let load_balancer = LoadBalancer { servers: servers_from_env };
    println!("{:?}", load_balancer.servers);

    let client: Arc<Client<HttpsConnector<HttpConnector>>> = Arc::new(Client::builder().build::<_, Body>(https));
    let arc = Arc::new(Mutex::new(load_balancer));
    let addr: SocketAddr = env!("LB_HOST").parse().unwrap();

    let make_svc = make_service_fn(|_conn| {
        let arc = Arc::clone(&arc);
        let client = Arc::clone(&client);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| router(req, Arc::clone(&client), Arc::clone(&arc))))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
