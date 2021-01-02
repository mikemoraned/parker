use actix_web::client::{Client, ClientBuilder};
use std::net::TcpListener;
use url::Url;

pub fn client() -> Client {
    ClientBuilder::new().disable_redirects().finish()
}

pub fn spawn_app(redirect_url: Url) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = parker::run(listener, redirect_url).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
