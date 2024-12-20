use elasticsearch::{BulkParts, CountParts, Elasticsearch, Error as ElasticsearchError, IndexParts, SearchParts};
use ethers::prelude::Transaction;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElasticRepositoryError {
    #[error("Erro de conexão com Elasticsearch: {0}")]
    ConnectionError(#[from] ElasticsearchError),
    #[error("Erro ao processar resposta: {0}")]
    ResponseError(String),
}

pub struct ElasticRepository {
    client: Elasticsearch,
}

impl ElasticRepository {
    pub fn new(elasticsearch_url: &str) -> Result<Self, ElasticRepositoryError> {
        let transport = elasticsearch::http::transport::TransportBuilder::new(
            elasticsearch::http::transport::SingleNodeConnectionPool::new(
                elasticsearch_url
                    .parse()
                    .map_err(|e| {
                        ElasticRepositoryError::ConnectionError(ElasticsearchError::from(e))
                    })
                    .expect("ERR "),
            ),
        )
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| ElasticRepositoryError::ConnectionError(ElasticsearchError::from(e)))?;

        let client = Elasticsearch::new(transport);
        Ok(Self { client })
    }

    /// Indexa um documento no Elasticsearch.
    ///
    /// # Argumentos
    ///
    /// * `index` - Nome do índice onde o documento será armazenado
    /// * `document` - Documento a ser indexado (deve implementar Serialize)
    ///
    /// # Exemplo
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let es_service = ElasticsearchService::new("http://localhost:9200");
    ///
    /// // Indexando um documento simples
    /// es_service.index_document(
    ///     "usuarios",
    ///     &json!({
    ///         "nome": "João Silva",
    ///         "idade": 30,
    ///         "email": "joao@exemplo.com"
    ///     })
    /// ).await?;
    /// ```
    pub async fn index_document<T: Serialize>(
        &self,
        index: &str,
        document: &T,
    ) -> Result<(), ElasticsearchError> {
        let _response = self
            .client
            .clone()
            .index(IndexParts::Index(index))
            .body(document)
            .send()
            .await?;
        Ok(())
    }

    pub async fn index_bulk_documents<T: Serialize>(
        &self,
        index: &str,
        documents: Vec<T>,
    ) -> Result<(), ElasticsearchError> {
        let mut bulk_body = Vec::new();

        for doc in documents {
            bulk_body.push(json!({
                "index": {
                    "_index": index,
                }
            }));

            bulk_body.push(serde_json::to_value(doc)?);
        }

        self.client
            .clone()
            .bulk(BulkParts::None)
            .body(
                bulk_body
                    .into_iter()
                    .map(|x| x.to_string().into_bytes())
                    .collect::<Vec<_>>(),
            )
            .send()
            .await?;

        Ok(())
    }



    /// Realiza uma busca no Elasticsearch.
    ///
    /// # Argumentos
    ///d
    /// * `index` - Nome do índice onde a busca será realizada
    /// * `query` - Query em formato JSON para a busca
    ///
    /// # Retorno
    ///
    /// Retorna um `Vec<T>` com os documentos encontrados, onde T deve implementar Deserialize
    ///
    /// # Exemplo
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let es_service = ElasticsearchService::new("http://localhost:9200");
    ///
    /// // Busca simples por todos os documentos
    /// let todos_docs = es_service.search::<serde_json::Value>(
    ///     "usuarios",
    ///     &json!({
    ///         "query": {
    ///             "match_all": {}
    ///         }
    ///     })
    /// ).await?;
    ///
    /// // Busca com filtros
    /// let docs_filtrados = es_service.search::<serde_json::Value>(
    ///     "usuarios",
    ///     &json!({
    ///         "query": {
    ///             "bool": {
    ///                 "must": [
    ///                     { "match": { "nome": "João" } },
    ///                     { "range": { "idade": { "gte": 18 } } }
    ///                 ]
    ///             }
    ///         }
    ///     })
    /// ).await?;
    /// ```
    pub async fn search<T: for<'de> Deserialize<'de>>(
        &self,
        index: &str,
        query: &Value,
    ) -> Result<Vec<T>, ElasticRepositoryError> {
        let response = self
            .client
            .search(SearchParts::Index(&[index]))
            .body(query)
            .send()
            .await?;

        let status = response.status_code();
        if !status.is_success() {
            return Err(ElasticRepositoryError::ResponseError(format!(
                "Falha na busca. Status: {}",
                status
            )));
        }

        let response_body = response.json::<Value>().await?;

        let hits = response_body["hits"]["hits"]
            .as_array()
            .ok_or_else(|| {
                ElasticRepositoryError::ResponseError(
                    "Resposta inválida do Elasticsearch".to_string(),
                )
            })?
            .iter()
            .filter_map(|hit| {
                let source = &hit["_source"];
                match serde_json::from_value::<T>(source.clone()) {
                    Ok(doc) => Some(doc),
                    Err(e) => {
                        log::warn!("Erro ao deserializar documento: {}", e);
                        None
                    }
                }
            })
            .collect();

        Ok(hits)
    }

    pub async fn index_documents_count(&self, index: &str) -> u64 {
        let response = self.client
            .count(CountParts::Index(&[index]))
            .send()
            .await
            .expect("Falha ao fazer a contagem");

        let response_body = response
            .json::<Value>()
            .await
            .expect("Falha ao interpretar a resposta");

        response_body["count"]
            .as_u64()
            .expect("Falha ao acessar o campo 'count'")
    }

}
