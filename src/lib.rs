use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use url::Url;

struct AppState {
    redirect_url: Url,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn redirect(data: web::Data<AppState>) -> HttpResponse {
    let location = data.redirect_url.as_str();
    HttpResponse::TemporaryRedirect()
        .header("Location", location)
        .body("Redirecting")
}

pub fn run(listener: TcpListener, redirect_url: Url) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                redirect_url: redirect_url.clone(),
            })
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .service(web::resource("/").to(redirect))
            .service(web::resource("/{rest:.+}").to(redirect))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
