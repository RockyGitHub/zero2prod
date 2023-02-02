use actix_web::{HttpServer, App, web, Responder, HttpResponse, dev::Server};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("0.0.0.0:8000")?
    .run();

    Ok(server)
}

