use actix_web::{web, App, HttpServer};
use unlock_mox_si_api::controller;
use unlock_mox_si_api::domain::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let load_config = config::Service::load();
    let load_config = match load_config {
        Ok(c) => c,
        Err(f) => {
            panic!("failed to load config: {}", f);
        }
    };
    let inject_config = web::Data::new(load_config);

    HttpServer::new(move || {
        App::new()
            .app_data(inject_config.clone())
            .service(controller::open::post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
