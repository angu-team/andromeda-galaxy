use std::collections::HashMap;
use crate::repositories::ethers_repository::EthersRepository;
use ethers::abi::{Abi, Address};
use ethers::middleware::Middleware;
use ethers::prelude::{Filter, Provider, Ws, H256, StreamExt, Transaction};
use std::sync::Arc;
use ethers::types::H160;
use log::Log;
use tokio::sync::RwLock;
use crate::http_client::HttpClient;

pub struct ListenContractEventsService {
    repository: Arc<RwLock<EthersRepository>>,
}

impl ListenContractEventsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>) -> Self {
        ListenContractEventsService { repository }
    }

    pub async fn exec(&self,user_id: i32, contract_address: String, event_signature: String,webhook:String){
        let repository = self.repository.clone();
        tokio::spawn(Self::spawn_process_task(repository, user_id, contract_address, event_signature,webhook));
    }

    async fn get_provider(
        repository: Arc<RwLock<EthersRepository>>,
        user_id: i32,
    ) -> Arc<Provider<Ws>> {
        repository.read().await.get_connection(user_id).unwrap()
    }

    fn create_event_filter(contract_address: Address, event_signature: &str) -> Filter {
        let event_signature_hash = H256::from_slice(&ethers::utils::keccak256(event_signature));
        Filter::new()
            .address(contract_address)
            .topic0(event_signature_hash)
    }

    async fn send_transaction(
        webhook: String,
        transaction: HashMap<H160, Transaction>,
    ) -> Result<(), reqwest::Error> {
        let client = HttpClient::new();

        client.get_client()
            .post(&webhook)
            .json(&transaction)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    async fn process_event(
        provider: Arc<Provider<Ws>>,
        hash: H256,
        webhook:String,
        address: H160
    ){
        let transaction_data = provider.get_transaction(hash).await;
        let mut contract_event = HashMap::new();

        match transaction_data {
            Ok(Some(value)) => {
                contract_event.insert(address, value);
                Self::send_transaction(webhook, contract_event).await.expect("TODO: panic message");
            }
            Ok(None) => {
                println!("Deu bom, mas tá vazio (None)!");
            }
            Err(e) => {
                println!("Deu ruim: {}", e);
            }
        }

    }

    fn spawn_process_task(
        repository: Arc<RwLock<EthersRepository>>,
        user_id: i32,
        contract_address: String,
        event_signature: String,
        webhook:String
    ) -> impl std::future::Future<Output = ()> {
        async move {
            let provider = Self::get_provider(repository.clone(), user_id).await;

            let contract_address: Address = contract_address
                .parse()
                .expect("Erro ao parsear o endereço do contrato");

            let filter = Self::create_event_filter(contract_address, &event_signature);

            let mut stream = match provider.subscribe_logs(&filter).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Erro ao criar stream de logs: {}", e);
                    return;
                }
            };

            println!("Escutando eventos: {}", event_signature);

            while let Some(log_result) = stream.next().await {
                let hash = log_result.transaction_hash.unwrap();
                Self::process_event(provider.clone(), hash, webhook.clone(),contract_address.clone()).await;
            }
        }
    }

}
