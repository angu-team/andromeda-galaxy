use crate::repositories::elastic_repository::ElasticRepository;
use crate::repositories::ethers_repository::EthersRepository;
use ethers::middleware::Middleware;
use ethers::prelude::BlockNumber;
use ethers::types::BlockId;
use futures::future::join_all;
use std::sync::{Arc, RwLock};
use tokio::task;

pub struct GetLogsService {
    repository: Arc<RwLock<EthersRepository>>,
    elastic_repository : Arc<RwLock<ElasticRepository>>
}

impl GetLogsService {

    pub fn new(repository: Arc<RwLock<EthersRepository>>,elastic_repository : Arc<RwLock<ElasticRepository>> ) -> Self{
        GetLogsService{
            repository,
            elastic_repository
        }
    }

    pub async fn exec(&self,user_id: i32,from_block: u64, to_block: u64) {
        let lock_repository = self.repository.read().unwrap();
        let provider = lock_repository.get_connection(user_id).expect("ERR");

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
                }
                _ => {}
            }
        }
        
    }

}