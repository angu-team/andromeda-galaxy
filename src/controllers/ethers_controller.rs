use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Route};
use serde::Deserialize;
use crate::services::ethers::apply_rpc::apply_rpc;

#[derive(Deserialize)]
struct ApplyRpcCtrl {
    user_id: i32,
    endpoint: String
}

async fn apply_rpc_ctrl(request: web::Json<ApplyRpcCtrl>) -> impl Responder {
    let user_id = request.user_id;
    let endpoint = request.endpoint.clone();

    apply_rpc(user_id,endpoint).await;
    HttpResponse::Ok()
}

pub fn ethers_routers() -> HashMap<String,Route>{
    let mut routes = HashMap::new();

    routes.insert(
        String::from("/"),web::post().to(apply_rpc_ctrl)
    );

    routes
}