mod controllers;
mod repositories;
mod services;

use crate::controllers::ethers_controller::EthersController;
use crate::repositories::ethers_repository::EthersRepository;
use crate::repositories::redis_repository::RedisRepository;
use crate::services::elasticsearch_service::ElasticsearchService;
use crate::services::ethers::apply_rpc_service::ApplyRpcService;

use dotenv::dotenv;
use std::env;
use std::sync::{Arc, RwLock};

use crate::services::ethers::get_logs_service::GetLogsService;
use actix_web::{web, App, HttpServer};
use redis::{Commands, FromRedisValue};

#[actix_web::main]
async fn main() {
    dotenv().ok();
    HttpServer::new(move || {
        let mut app = App::new();

        let redis_repository = Arc::new(RwLock::new(RedisRepository::connect()));
        let ethers_repository = Arc::new(RwLock::new(EthersRepository::new()));

        let apply_rpc_service = Arc::new(ApplyRpcService::new(
            ethers_repository.clone(),
            redis_repository.clone(),
        ));

        let elasticsearch_service = Arc::new(
            ElasticsearchService::new(
                env::var("ELASTICSEARCH_URI")
                    .expect("ELASTICSEARCH_URI not set")
                    .as_str(),
            )
            .expect("Falha ao criar ElasticsearchService"),
        );

        let get_logs_service = Arc::new(GetLogsService::new(ethers_repository.clone()));

        app = app.app_data(web::Data::new(apply_rpc_service.clone()));
        app = app.app_data(web::Data::new(get_logs_service.clone()));
        app = app.app_data(web::Data::new(elasticsearch_service.clone()));

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
