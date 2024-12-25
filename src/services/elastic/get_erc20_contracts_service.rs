use crate::repositories::elastic_repository::ElasticRepository;
use ethers::types::Transaction;
use serde_json::json;
use std::sync::Arc;

pub struct GetErc20ContractsService {
    repository: Arc<ElasticRepository>,
}

impl GetErc20ContractsService {
    pub fn new(repository: Arc<ElasticRepository>) -> Self {
        GetErc20ContractsService { repository }
    }

    pub async fn exec(&self) {
        // Define a query para obter apenas a contagem
        let query = json!({
          "query": {
            "bool": {
              "must": [
                { "match": { "input": "a9059cbb" } },
                { "match": { "input": "70a08231" } },
                { "match": { "input": "dd62ed3e" } },
                { "match": { "input": "095ea7b3" } },
                { "match": { "input": "23b872dd" } },
                { "match": { "input": "f2fde38b" } },
                { "match": { "input": "715018a6" } },
              ]
            }
          }
        });
        // let count = self.repository.index_documents_count("transactions",&query).await;
        let x:Vec<Transaction> = self.repository.search("transactions", &query).await.expect("TODO: panic message");
        println!("{}",x.len())
        // println!("{count}");
        // let response: Vec<Transaction> = self
        //     .repository
        //     .search("transactions", &query)
        //     .await
        //     .expect("ERR");
        // println!("{:?}", response.len())
    }
}
