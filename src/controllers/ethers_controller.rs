use crate::services::ethers::apply_rpc_service::ApplyRpcService;
use actix_web::{web, HttpResponse, Responder, Route};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::services::ethers::call_functions_service::CallFunctionsService;
use crate::services::ethers::get_logs_service::GetLogsService;
use crate::services::ethers::listen_contract_event_service::ListenContractEventsService;
use crate::services::ethers::listen_deploy_erc20_contracts_service::ListenDeployErc20ContractsService;

pub struct EthersController;

#[derive(Deserialize)]
struct ApplyRpcCtrl {
    endpoint: String,
}

#[derive(Deserialize)]
struct CallFunctionsCtrl {
    functions_name: Vec<String>,
    abi:String,
    address:String
}

#[derive(Deserialize)]
struct GetLogsCtrl {
    from_block: u64,
    to_block: u64
}

#[derive(Deserialize)]
struct ListenContractEventsCtrl {
    address: String,
    webhook:String,
    event_signature: String,
}

#[derive(Deserialize)]
struct ListenDeployErc20ContractsCtrl {
    webhook:String
}


#[derive(Deserialize)]
struct PathParams {
    id: i32
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

    pub async fn listen_contract_events_ctrl(
        path: web::Path<PathParams>,
        request: web::Json<ListenContractEventsCtrl>,
        service: web::Data<Arc<ListenContractEventsService>>,
    ) -> impl Responder {
        let user_id = path.id;
        let contract_address = request.address.clone();
        let event_signature = request.event_signature.clone();
        let webhook = request.webhook.clone();

        service.exec(user_id,contract_address,event_signature,webhook).await;
        HttpResponse::Ok()
    }

    pub async fn call_functions_ctrl(
        path: web::Path<PathParams>,
        request: web::Json<CallFunctionsCtrl>,
        service: web::Data<Arc<CallFunctionsService>>
    ) -> impl Responder {
        let id = path.id.clone();
        let functions_name = request.functions_name.clone();
        let abi = request.abi.clone();
        let address = request.address.clone();

        let service_response = service.exec(id, address,functions_name,abi).await;
        // let service_response = web::Json(service_response);
        HttpResponse::Ok().json(service_response)
    }

    pub async fn listen_deploy_erc20_contracts_ctrl(
        path: web::Path<PathParams>,
        request: web::Json<ListenDeployErc20ContractsCtrl>,
        service: web::Data<Arc<ListenDeployErc20ContractsService>>
    ) -> impl Responder {
        let id = path.id.clone();
        let webhook = request.webhook.clone();

        service.exec(id,webhook).await;
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
        routes.insert(String::from("ethers/{id}/call_functions"), web::post().to(Self::call_functions_ctrl));
        routes.insert(String::from("ethers/{id}/get_logs"), web::post().to(Self::get_logs_ctrl));
        routes.insert(String::from("ethers/{id}/listen_deploy_erc20"), web::post().to(Self::listen_deploy_erc20_contracts_ctrl));
        routes.insert(String::from("ethers/{id}/listen_contract_events"), web::post().to(Self::listen_contract_events_ctrl));

        routes
    }
}
