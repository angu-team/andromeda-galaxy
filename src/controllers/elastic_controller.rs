use crate::services::elastic::get_erc20_contracts_service::GetErc20ContractsService;
use crate::services::elastic::get_labels_service::GetLabelsService;
use actix_web::{web, HttpResponse, Responder, Route};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ElasticController;

#[derive(Deserialize)]
struct GetLabelsFromAddressCtrl {
    address: String,
}

impl ElasticController {
    pub fn new() -> Self {
        ElasticController
    }

    pub async fn get_erc20_contracts_ctrl(
        service: web::Data<Arc<GetErc20ContractsService>>,
    ) -> impl Responder {
        service.exec().await;
        HttpResponse::Ok()
    }

    pub async fn get_labels_from_address_ctrl(
        path: web::Path<GetLabelsFromAddressCtrl>,
        service: web::Data<Arc<GetLabelsService>>,
    ) -> impl Responder {
        let address = path.address.clone();

        match service.exec_by_address(address).await {
            Ok(labels) => {
                if labels.is_empty() {
                    HttpResponse::NotFound().json(serde_json::json!({ "error": "NOT_FOUND" }))
                } else {
                    HttpResponse::Ok().json(&labels[0])
                }
            }
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    pub fn routes(self) -> HashMap<String, Route> {
        let mut routes = HashMap::new();

        routes.insert(
            String::from("elastic/erc20_contracts"),
            web::get().to(Self::get_erc20_contracts_ctrl),
        );

        routes.insert(
            String::from("elastic/labels_from_address/{address}"),
            web::get().to(Self::get_labels_from_address_ctrl),
        );

        routes
    }
}
