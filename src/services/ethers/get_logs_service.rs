use crate::repositories::elastic_repository::ElasticRepository;
use crate::repositories::ethers_repository::EthersRepository;
use ethers::middleware::Middleware;
use ethers::prelude::{BlockNumber, Transaction};
use ethers::types::{Block, BlockId};
use futures::future::join_all;
use serde_json::json;
use std::sync::{Arc, RwLock};
use tokio::sync::{Semaphore, SemaphorePermit};
use tokio::task;

#[derive(Clone)]
pub struct GetLogsService {
    repository: Arc<RwLock<EthersRepository>>,
    elastic_repository: Arc<ElasticRepository>,
}

impl GetLogsService {
    pub fn new(
        repository: Arc<RwLock<EthersRepository>>,
        elastic_repository: Arc<ElasticRepository>,
    ) -> Self {
        GetLogsService {
            repository,
            elastic_repository,
        }
    }

    async fn save_tx_if_block_not_saved(&self, block: Block<Transaction>) {
        // let lock_elastic = self.elastic_repository.;

        let docs_filtrados = self.elastic_repository
            .search::<serde_json::Value>(
                "transactions",
                &json!({
                    "query": {
                        "bool": {
                            "must": [
                                { "match": {"blockHash": block.hash}},
                            ]
                        }
                    }
                }),
            )
            .await
            .expect("ERR");

        if docs_filtrados.len() == 0 {
            self.elastic_repository
                .index_bulk_documents("transactions", block.transactions)
                .await
                .expect("TODO: panic message");
        }
    }

    pub async fn exec(&self, user_id: i32, from_block: u64, to_block: u64) {
        let mut handles = Vec::new();
        let semaphore = Arc::new(Semaphore::new(300));

        for block_number in from_block..=to_block {
            let provider = self.repository.read().unwrap().get_connection(user_id).expect("ERR ");
            let semaphore = Arc::clone(&semaphore);
            let my_clone = self.clone();

            let handle = task::spawn(async move {
                let lock:SemaphorePermit = semaphore.acquire().await.unwrap();

                let block_id = BlockId::Number(BlockNumber::Number(block_number.into()));
                let block = provider.get_block_with_txs(block_id).await.expect("ERR").unwrap();

                my_clone.save_tx_if_block_not_saved(block).await;
                println!("Block {block_number} read!");
            });

            handles.push(handle);
        }

    }
}
