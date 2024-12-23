use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder, Route};
use crate::services::elastic::get_erc20_contracts_service::GetErc20ContractsService;

pub struct ElasticController;

impl ElasticController {
    pub fn new() -> Self {ElasticController}

    pub async fn get_erc20_contracts_ctrl(
        service: web::Data<Arc<GetErc20ContractsService>>,
    ) -> impl Responder {
        service.exec().await;
        HttpResponse::Ok()
    }

    pub fn routes(self) -> HashMap<String, Route> {
        let mut routes = HashMap::new();

        routes.insert(String::from("elastic/erc20_contracts"), web::get().to(Self::get_erc20_contracts_ctrl));

        routes
    }

}