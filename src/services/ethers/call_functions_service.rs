use ethers::abi::{Abi, Token};
use ethers::contract::{Contract, ContractInstance};
use ethers::middleware::Middleware;
use ethers::prelude::{Provider, Ws};
use ethers::types::Address;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::repositories::ethers_repository::EthersRepository;
use crate::utils::ethers_utils::EthersUtils;

pub struct CallFunctionsService {
    repository: Arc<RwLock<EthersRepository>>,
}

impl CallFunctionsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>) -> Self {
        CallFunctionsService { repository }
    }

    async fn get_provider(&self, user_id: i32) -> Arc<Provider<Ws>> {
        let lock = self.repository.read().await;
        lock.get_connection(user_id).expect("Erro na obtenção da conexão com o Provider")
    }

    fn create_contract(
        provider: Arc<Provider<Ws>>,
        contract_address: String,
        abi: String,
    ) -> ContractInstance<Arc<Provider<Ws>>, Provider<Ws>> {
        let parsed_abi: Abi = serde_json::from_str(&abi).expect("Erro ao parsear o ABI");
        let token_address: Address = contract_address.parse().expect("Erro ao parsear Address");
        Contract::new(token_address, parsed_abi, provider)
    }

    async fn get_call_response(
        contract: ContractInstance<Arc<Provider<Ws>>, Provider<Ws>>,
        function_name: &str,
    ) -> Option<Token> {
        match contract.method(&function_name, ()) {
            Ok(method) => method.call().await.ok(),
            Err(_) => None,
        }
    }

    pub async fn exec(
        &self,
        user_id: i32,
        contract_address: String,
        function_names: Vec<String>,
        abi: String,
    ) -> HashMap<String, Value> {
        let provider = self.get_provider(user_id).await;
        let contract = Self::create_contract(provider.clone(), contract_address, abi);
        let mut functions_response: HashMap<String, Value> = HashMap::new();

        for function_name in function_names {
            if let Some(token) = Self::get_call_response(contract.clone(), &function_name).await {
                let token_to_json = EthersUtils::token_to_json(token);
                functions_response.insert(function_name, token_to_json);
            }
        }

        functions_response
    }
}