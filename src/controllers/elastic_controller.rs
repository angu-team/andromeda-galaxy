use crate::services::elastic::get_erc20_contracts_service::GetErc20ContractsService;
use crate::services::elastic::get_labels_service::GetLabelsService;
use crate::services::elastic::get_transactions_service::GetTransactionsService;
use actix_web::{web, HttpResponse, Responder, Route};
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub struct ElasticController;

fn deserialize_comma_separated<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.split(',')
        .map(|s| s.trim().parse().map_err(de::Error::custom))
        .collect()
}

#[derive(Deserialize)]
struct GetLabelsFromAddressCtrl {
    address: String,
}

#[derive(Deserialize)]
struct GetTransactionsCtrl {
    #[serde(default, deserialize_with = "deserialize_comma_separated")]
    inputs: Vec<String>,
    cursor: Option<String>,
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

    pub async fn get_transactions_ctrl(
        query: web::Query<GetTransactionsCtrl>,
        service: web::Data<Arc<GetTransactionsService>>,
    ) -> impl Responder {
        let cursor = query.cursor.clone();
        let inputs = query.inputs.clone();

        // let inputs = vec![
        //     "a9059cbb".to_string(),
        //     "70a08231".to_string(),
        //     "dd62ed3e".to_string(),
        //     "095ea7b3".to_string(),
        //     "23b872dd".to_string(),
        //     "f2fde38b".to_string(),
        //     "715018a6".to_string(),
        // ];

        match service.exec(inputs, cursor).await {
            Ok(transactions) => HttpResponse::Ok().json(&transactions),
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

        // Exemplo da requisição:
        // http://localhost:8080/elastic/transactions?inputs=a9059cbb,70a08231,dd62ed3e,095ea7b3,23b872dd,f2fde38b,715018a6
        routes.insert(
            String::from("elastic/transactions"),
            web::get().to(Self::get_transactions_ctrl),
        );

        routes
    }
}
