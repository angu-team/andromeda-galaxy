mod controllers;
mod repositories;
mod services;

use crate::controllers::ethers_controller::EthersController;
use crate::repositories::ethers_repository::EthersRepository;
use crate::repositories::redis_repository::RedisRepository;
use crate::services::ethers::apply_rpc_service::ApplyRpcService;
use std::sync::{Arc, RwLock};

use actix_web::{web, App, HttpServer};
use redis::{Commands, FromRedisValue};

#[actix_web::main]
async fn main() {
    HttpServer::new(move || {
        let mut app = App::new();

        let redis_repository = Arc::new(RwLock::new(RedisRepository::connect()));
        let ethers_repository = Arc::new(RwLock::new(EthersRepository::new()));

        let apply_rpc_service = Arc::new(ApplyRpcService::new(
            ethers_repository.clone(),
            redis_repository.clone(),
        ));

        app = app.app_data(web::Data::new(apply_rpc_service.clone()));

        let ethers_controller = EthersController::new();
        for (endpoint, route) in ethers_controller.routes() {
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
