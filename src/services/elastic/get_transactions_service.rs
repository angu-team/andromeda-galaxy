use crate::repositories::elastic_repository::{ElasticRepository, SearchResult};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    hash: String,
    from: String,
    to: Option<String>,
    input: String,
    // ... outros campos necessários ...
}

pub struct GetTransactionsService {
    elastic_repository: Arc<ElasticRepository>,
}

impl GetTransactionsService {
    pub fn new(elastic_repository: Arc<ElasticRepository>) -> Self {
        GetTransactionsService { elastic_repository }
    }

    pub async fn exec(
        &self,
        inputs: Vec<String>,
        cursor: Option<String>,
    ) -> Result<SearchResult<Transaction>, Box<dyn std::error::Error>> {
        let must_clauses: Vec<_> = inputs
            .iter()
            .map(|input| json!({ "match": { "input": input } }))
            .collect();

        let query = json!({
            "bool": {
                "must": must_clauses
            }
        });

        let result = self
            .elastic_repository
            .search_with_pagination::<Transaction>("transactions", Some(query), 1000, cursor)
            .await?;

        Ok(result)
    }
}
