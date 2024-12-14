mod controllers;
mod repositories;
mod services;

use crate::controllers::ethers_controller::ethers_routers;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() {

    HttpServer::new(move || {
        let ethers_routers = ethers_routers();
        let mut app = App::new();

        for (endpoint, route) in ethers_routers {
            app = app.route(&endpoint, route);
        }

        app
    })
    .bind("127.0.0.1:8080")
    .expect("ERR")
    .run()
    .await
    .expect("TODO: panic message");
}
