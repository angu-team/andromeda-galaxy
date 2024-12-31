use crate::http_client::HttpClient;
use crate::repositories::ethers_repository::EthersRepository;
use crate::utils::bytecode_utils::BytecodeUtils;
use ethers::prelude::{Block, BlockNumber, Provider, StreamExt, Ws, H256};
use ethers::providers::Middleware;
use ethers::types::{BlockId, Transaction};
use std::sync::{Arc};
use tokio::sync::mpsc::Receiver;
use tokio::sync::RwLock;

pub struct ListenDeployErc20ContractsService {
    repository: Arc<RwLock<EthersRepository>>,
    http_client: HttpClient,
}

impl ListenDeployErc20ContractsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>, http_client: HttpClient) -> Self {
        ListenDeployErc20ContractsService {
            repository,
            http_client,
        }
    }

    pub async fn exec(&self, user_id: i32, webhook: String) {
        let block_listener = self.repository.write().await.get_block_listener(user_id);

        if let Some(receiver) = block_listener {
            let repository = self.repository.clone();
            tokio::spawn(Self::spawn_process_task(repository, user_id, webhook, receiver));
        } else {
            println!("Nenhum listener configurado para o user_id: {}", user_id);
        }
    }

    async fn process_block(
        provider: &Arc<Provider<Ws>>,
        block: Block<H256>,
    ) -> Vec<Transaction> {
        let block_id = BlockId::Number(BlockNumber::Number(block.number.unwrap()));
        let block_data = provider
            .get_block_with_txs(block_id)
            .await
            .expect("ERR")
            .unwrap();

        let mut transactions = Vec::new();

        for transaction in block_data.transactions {
            let bytecode_is_deploy_erc20 =
                BytecodeUtils::bytecode_is_deploy_erc20(transaction.input.to_string());
            if transaction.to.is_none() && bytecode_is_deploy_erc20 {
                transactions.push(transaction);
            }
        }

        transactions
    }

    async fn send_transactions(
        webhook: String,
        transactions: Vec<Transaction>,
    ) -> Result<(), reqwest::Error> {
        let client = HttpClient::new();

        client.get_client()
            .post(&webhook)
            .json(&transactions)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    fn spawn_process_task(
        repository: Arc<RwLock<EthersRepository>>,
        user_id: i32,
        webhook: String,
        mut receiver: Receiver<Block<H256>>,
    ) -> impl std::future::Future<Output = ()> {
        async move {
            let provider = Self::get_provider(repository, user_id).await;

            while let Some(block) = receiver.recv().await {
                println!("Block: {}", block.number.unwrap());
                let transactions = Self::process_block(&provider, block).await;

                if !transactions.is_empty() {
                    if let Err(err) = Self::send_transactions(webhook.clone(), transactions).await {
                        eprintln!("Erro ao enviar requisição: {}", err);
                    }
                }
            }
        }
    }

    async fn get_provider(
        repository: Arc<RwLock<EthersRepository>>,
        user_id: i32,
    ) -> Arc<Provider<Ws>> {
        repository.read().await.get_connection(user_id).unwrap()
    }

}
