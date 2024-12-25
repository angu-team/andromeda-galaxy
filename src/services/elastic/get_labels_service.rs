use crate::repositories::elastic_repository::ElasticRepository;
use crate::repositories::elastic_repository::SearchResult;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct ElasticLabel {
    address: String,
    chain_id: u32,
    label: String,
    name_tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    hash: String,
    from: String,
    to: Option<String>,
    input: String,
    // ... outros campos necessários ...
}

pub struct GetLabelsService {
    elastic_repository: Arc<ElasticRepository>,
}

impl GetLabelsService {
    pub fn new(elastic_repository: Arc<ElasticRepository>) -> Self {
        GetLabelsService { elastic_repository }
    }

    /// Busca labels no Elasticsearch.
    ///
    /// # Argumentos
    ///
    /// * `label` - Label a ser buscada (opcional) passe None para buscar todos
    ///
    /// # Exemplo
    ///
    /// ```rust
    /// let service = GetLabelsService::new(elastic_repository);
    ///
    /// // Buscar todas as labels
    /// let all_labels = service.exec(None).await?;
    ///
    /// // Buscar labels específicas
    /// let specific_labels = service.exec(Some("exchange".to_string())).await?;
    /// ```
    pub async fn exec(
        &self,
        label: Option<String>,
        size: i32,
        cursor: Option<String>,
    ) -> Result<SearchResult<ElasticLabel>, Box<dyn std::error::Error>> {
        let query = match label {
            None => json!({
                "query": {
                    "match_all": {}
                }
            }),
            Some(label) => json!({
                "query": {
                    "match": {
                        "label": label
                    }
                }
            }),
        };

        let result = self
            .elastic_repository
            .search_with_pagination::<ElasticLabel>(
                "labels",
                Some(query.to_string()),
                size as i64,
                cursor,
            )
            .await?;
        Ok(result)
    }
}
