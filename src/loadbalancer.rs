use hyper::{Body, Client, Error, Response};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

#[derive(Debug)]
pub struct LoadBalancer {
    pub servers: Vec<String>,
}

impl LoadBalancer {
    pub fn shuffle_servers(&mut self) {
        self.servers.rotate_left(1);
    }

    pub async fn redirect(&mut self, client: &Client<HttpsConnector<HttpConnector>>, path: &str) -> Result<Response<Body>, Error> {
        self.shuffle_servers();
        println!("Redirecting to server: {}, Path: {}", self.servers[0], path);
        let uri = format!("{}{}", self.servers[0], path).parse::<hyper::Uri>().unwrap();
        println!("URI: {}", &uri);
        let res = client.get(uri).await;

        println!("{:?}", res.as_ref().unwrap().body());
        res
    }
}
