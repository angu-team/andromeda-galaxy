mod controllers;
pub mod http_client;
mod repositories;
mod services;
mod utils;
use crate::controllers::ethers_controller::EthersController;
use crate::repositories::elastic_repository::ElasticRepository;
use crate::repositories::ethers_repository::EthersRepository;
use crate::repositories::redis_repository::RedisRepository;
use crate::services::ethers::apply_rpc_service::ApplyRpcService;

use dotenv::dotenv;
use services::elastic::get_labels_service::GetLabelsService;
use std::env;
use std::sync::{Arc, RwLock};

use crate::controllers::elastic_controller::ElasticController;
use crate::services::elastic::get_erc20_contracts_service::GetErc20ContractsService;
use crate::services::ethers::get_logs_service::GetLogsService;
use crate::services::ethers::listen_deploy_erc20_contracts_service::ListenDeployErc20ContractsService;
use actix_web::{middleware::Logger, web, App, HttpServer};
use http_client::HttpClient;
use redis::{Commands, FromRedisValue};

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    HttpServer::new(move || {
        let mut app = App::new().wrap(Logger::default());
        let http_client = HttpClient::new();
        let elastic_repository = Arc::new(
            ElasticRepository::new(
                env::var("ELASTICSEARCH_URI")
                    .expect("ELASTICSEARCH_URI not set")
                    .as_str(),
            )
            .expect("Falha ao criar ElasticsearchService"),
        );

        let redis_repository = Arc::new(RedisRepository::connect());
        let ethers_repository = Arc::new(RwLock::new(EthersRepository::new()));

        let get_erc20_contracts =
            Arc::new(GetErc20ContractsService::new(elastic_repository.clone()));

        let apply_rpc_service = Arc::new(ApplyRpcService::new(
            ethers_repository.clone(),
            redis_repository.clone(),
        ));

        let get_labels_service = Arc::new(GetLabelsService::new(elastic_repository.clone()));

        let get_logs_service = Arc::new(GetLogsService::new(
            ethers_repository.clone(),
            elastic_repository.clone(),
        ));

        let listen_deploy_erc20_contracts_service = Arc::new(
            ListenDeployErc20ContractsService::new(ethers_repository.clone(),http_client),
        );

        app = app.app_data(web::Data::new(apply_rpc_service.clone()));
        app = app.app_data(web::Data::new(get_erc20_contracts.clone()));
        app = app.app_data(web::Data::new(get_labels_service.clone()));
        app = app.app_data(web::Data::new(get_logs_service.clone()));
        app = app.app_data(web::Data::new(
            listen_deploy_erc20_contracts_service.clone(),
        ));

        let ethers_controller = EthersController::new();
        let elastic_controller = ElasticController::new();

        for (endpoint, route) in ethers_controller.routes() {
            app = app.route(&endpoint, route);
        }

        for (endpoint, route) in elastic_controller.routes() {
            app = app.route(&endpoint, route);
        }

        app
    })
    .bind("0.0.0.0:8080")
    .expect("ERR")
    .run()
    .await
    .expect("TODO: panic message");
}
