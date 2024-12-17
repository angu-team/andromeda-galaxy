use crate::services::ethers::apply_rpc_service::ApplyRpcService;
use actix_web::{web, HttpResponse, Responder, Route};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use crate::services::ethers::get_logs_service::GetLogsService;

pub struct EthersController;

#[derive(Deserialize)]
struct ApplyRpcCtrl {
    endpoint: String,
}

#[derive(Deserialize)]
struct GetLogsCtrl {
    from_block: u64,
    to_block: u64
}

#[derive(Deserialize)]
struct PathParams {
    id: i32,
}

impl EthersController {
    pub fn new() -> Self {
        EthersController {}
    }

    pub async fn get_logs_ctrl(
        path: web::Path<PathParams>,
        request: web::Json<GetLogsCtrl>,
        service: web::Data<Arc<GetLogsService>>,
    ) -> impl Responder {
        let user_id = path.id;

        let from_block = request.from_block;
        let to_block = request.to_block;

        service.exec(user_id,from_block,to_block).await;
        HttpResponse::Ok()
    }

    pub async fn apply_rpc_ctrl(
        path: web::Path<PathParams>,
        request: web::Json<ApplyRpcCtrl>,
        service: web::Data<Arc<ApplyRpcService>>,
    ) -> impl Responder {
        let endpoint = request.endpoint.clone();
        let user_id = path.id;

        service.exec(user_id, endpoint).await;
        HttpResponse::Ok()
    }

    pub fn routes(self) -> HashMap<String, Route> {
        let mut routes = HashMap::new();

        routes.insert(String::from("ethers/{id}/apply_rpc"), web::post().to(Self::apply_rpc_ctrl));
        routes.insert(String::from("ethers/{id}/get_logs"), web::post().to(Self::get_logs_ctrl));

        routes
    }
}
