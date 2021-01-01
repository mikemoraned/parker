use clap::{App, Arg};
use env_logger::Env;
use parker::run;
use std::net::TcpListener;
use url::Url;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn validate_url(s: String) -> Result<(), String> {
    match Url::parse(&s) {
        Ok(_url) => Ok(()),
        Err(parse_error) => Err(parse_error.to_string()),
    }
}

fn validate_port(s: String) -> Result<(), String> {
    match s.parse::<u16>() {
        Ok(port) => {
            if port > 0 {
                Ok(())
            } else {
                Err("port must be positive".to_string())
            }
        }
        Err(parse_error) => Err(parse_error.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let matches = App::new("parker")
        .version(VERSION)
        .about("Helps park websites")
        .author(AUTHORS)
        .arg(
            Arg::with_name("redirect-url")
                .short("r")
                .long("redirect-url")
                .help("Sets the URL to redirect to e.g. `http://example.com/`")
                .required(true)
                .validator(validate_url)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Sets the port to listen on")
                .required(false)
                .validator(validate_port)
                .takes_value(true),
        )
        .get_matches();

    let redirect_url = Url::parse(matches.value_of("redirect-url").unwrap()).unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    log::info!(
        "starting, with redirect_url={}, port={}",
        redirect_url,
        port
    );
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind port");
    run(listener, redirect_url)?.await
}
