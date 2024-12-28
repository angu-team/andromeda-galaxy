use crate::http_client::HttpClient;
use crate::repositories::ethers_repository::EthersRepository;
use crate::utils::bytecode_utils::BytecodeUtils;
use ethers::prelude::{BlockNumber, StreamExt};
use ethers::providers::Middleware;
use ethers::types::{BlockId, Transaction};
use std::sync::{Arc, RwLock};

pub struct ListenDeployErc20ContractsService {
    repository: Arc<RwLock<EthersRepository>>,
    http_client: HttpClient,
}

impl ListenDeployErc20ContractsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>, http_client: HttpClient) -> Self{
        ListenDeployErc20ContractsService { repository,http_client}
    }

    pub async fn exec(&self, user_id:i32, webhook:String){
        let lock_provider = Arc::new(self.repository.read().unwrap());

        let provider = lock_provider.clone().get_connection(user_id).unwrap();
        let stream = provider.subscribe_blocks().await.unwrap();

        stream.for_each(|block| {
            let provider = lock_provider.clone().get_connection(user_id).unwrap();

            async move {
                let block_id = BlockId::Number(BlockNumber::Number(block.number.unwrap()));
                let block_data = provider.get_block_with_txs(block_id).await.expect("ERR").unwrap();
                let mut transactions:Vec<Transaction> = Vec::new();

                for transaction in block_data.transactions {
                    let bytecode_is_deploy_erc20 = BytecodeUtils::bytecode_is_deploy_erc20(transaction.input.to_string());

                    if(transaction.to.is_none() && bytecode_is_deploy_erc20) {
                        println!("{:?}",transaction.hash);
                        transactions.push(transaction);
                    }
                }
                let length = transactions.len();
                if(length >= 1){
                    match self.http_client.get_client().post(webhook).json(&transactions).send().await {
                        Ok(resp) => Some(resp),
                        Err(err) => {
                            eprintln!("Erro ao enviar requisição: {}", err);
                            None
                        }
                    };
                }


            }
        }).await;

    }

}