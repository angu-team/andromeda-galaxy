use crate::repositories::elastic_repository::ElasticRepository;
use crate::repositories::ethers_repository::EthersRepository;
use elasticsearch::http::request::JsonBody;
use ethers::core::k256::elliptic_curve::bigint::Encoding;
use ethers::core::k256::U256;
use ethers::middleware::Middleware;
use ethers::prelude::{BlockNumber, Transaction};
use ethers::types::{Block, BlockId};
use ethers::utils::hex::hex;
use ethers::utils::keccak256;
use futures::future::join_all;
use serde_json::json;
use std::future::Future;
use std::sync::{Arc, RwLock};
use tokio::task;

pub struct GetLogsService {
    repository: Arc<RwLock<EthersRepository>>,
    elastic_repository: Arc<RwLock<ElasticRepository>>,
}

impl GetLogsService {
    pub fn new(
        repository: Arc<RwLock<EthersRepository>>,
        elastic_repository: Arc<RwLock<ElasticRepository>>,
    ) -> Self {
        GetLogsService {
            repository,
            elastic_repository,
        }
    }

    async fn save_tx_if_block_not_saved(&self, block: Block<Transaction>) {
        let lock_elastic = self.elastic_repository.write().unwrap();
        //
        // let docs_filtrados = lock_elastic
        //     .search::<serde_json::Value>(
        //         "transactions",
        //         &json!({
        //             "query": {
        //                 "bool": {
        //                     "must": [
        //                         { "match": {"blockHash": block.hash}},
        //                     ]
        //                 }
        //             }
        //         }),
        //     )
        //     .await
        //     .expect("ERR");
        //
        // if docs_filtrados.len() == 0 {
            lock_elastic
                .index_bulk_documents("transactions", block.transactions)
                .await
                .expect("TODO: panic message");
        // }
    }

    pub async fn exec(&self, user_id: i32, from_block: u64, to_block: u64) {
        let lock_provider = self.repository.read().unwrap();
        let provider = lock_provider.get_connection(user_id).expect("ERR ");

        let mut handles = Vec::new();

        for block_number in from_block..=to_block {
            let provider = provider.clone();

            let handle = task::spawn(async move {
                let block_id = BlockId::Number(BlockNumber::Number(block_number.into()));
                let logs = provider.get_block_with_txs(block_id).await;
                println!("Block {block_number} read!");
                logs
            });

            handles.push(handle);
        }

        let results = join_all(handles).await;

        for result in results {
            match result {
                Ok(Ok(logs)) => {
                    let unwrap = logs.unwrap();
                    let hash = unwrap.transactions.len();

                    let number = unwrap.number.expect("Err");
                    println!("Block {number} has {hash} transactions");

                    self.save_tx_if_block_not_saved(unwrap).await;
                }
                _ => {}
            }
        }

    }
}
