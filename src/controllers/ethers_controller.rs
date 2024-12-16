use crate::services::ethers::apply_rpc::ApplyRpcService;
use actix_web::{web, HttpResponse, Responder, Route};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

pub struct EthersController;

#[derive(Deserialize)]
struct ApplyRpcCtrl {
    user_id: i32,
    endpoint: String,
}

impl EthersController {
    pub fn new() -> Self {
        EthersController {}
    }

    pub async fn apply_rpc_ctrl(
        request: web::Json<ApplyRpcCtrl>,
        service: web::Data<Arc<ApplyRpcService>>,
    ) -> impl Responder {
        let user_id = request.user_id;
        let endpoint = request.endpoint.clone();

        service.exec(user_id, endpoint).await;
        HttpResponse::Ok()
    }

    pub fn routes(self) -> HashMap<String, Route> {
        let mut routes = HashMap::new();

        routes.insert(String::from("/"), web::post().to(Self::apply_rpc_ctrl));
        routes
    }
}
