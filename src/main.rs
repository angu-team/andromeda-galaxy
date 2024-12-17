mod controllers;
mod repositories;
mod services;

use crate::controllers::ethers_controller::EthersController;
use crate::repositories::ethers_repository::EthersRepository;
use crate::repositories::redis_repository::RedisRepository;
use crate::repositories::elastic_repository::ElasticRepository;
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

        let elastic_repository = Arc::new(RwLock::new(
            ElasticRepository::new(
                env::var("ELASTICSEARCH_URI")
                    .expect("ELASTICSEARCH_URI not set")
                    .as_str(),
            )
            .expect("Falha ao criar ElasticsearchService"),
        ));

        let redis_repository = Arc::new(RwLock::new(RedisRepository::connect()));
        let ethers_repository = Arc::new(RwLock::new(EthersRepository::new()));

        let apply_rpc_service = Arc::new(ApplyRpcService::new(
            ethers_repository.clone(),
            redis_repository.clone(),
        ));

        let get_logs_service = Arc::new(GetLogsService::new(ethers_repository.clone(),elastic_repository.clone()));

        app = app.app_data(web::Data::new(apply_rpc_service.clone()));
        app = app.app_data(web::Data::new(get_logs_service.clone()));

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
