use parker::run;
use std::net::TcpListener;

use url::Url;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redirect_url = Url::parse("http://example.com/").unwrap();
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port");
    run(listener, redirect_url)?.await
}
