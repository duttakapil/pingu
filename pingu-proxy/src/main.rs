use hyper::{Client, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper_tls::HttpsConnector;
use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    target_url: String,
}

fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_contents = fs::read_to_string("proxy-config.json")?;
    let config: Config = serde_json::from_str(&config_contents)?;
    Ok(config)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config()?;
    let target_url: hyper::Uri = config.target_url.parse().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    
    let make_svc = make_service_fn(move |_| {
        let client = create_https_client();
        let target = target_url.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, client.to_owned(), target.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

fn create_https_client() -> Client<HttpsConnector<hyper::client::HttpConnector>> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

async fn handle_request(
    req: Request<hyper::Body>,
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    target_url: hyper::Uri,
) -> Result<Response<hyper::Body>, hyper::Error> {
    let (mut parts, body) = req.into_parts();
    parts.uri = target_url;

    let req = Request::from_parts(parts, body);

    let resp = client.request(req).await?;
    Ok(resp)
}
