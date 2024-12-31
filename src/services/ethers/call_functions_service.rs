use ethers::abi::Abi;
use ethers::abi::Token;
use ethers::contract::{Contract, ContractInstance};
use ethers::middleware::Middleware;
use std::collections::HashMap;
use std::sync::{Arc};
use ethers::prelude::{Provider, Ws};
use crate::repositories::ethers_repository::EthersRepository;
use crate::utils::abi_utils::AbiUtils;
use crate::utils::ethers_utils::EthersUtils;
use ethers::types::Address;
use serde_json::Value;
use tokio::sync::RwLock;

pub struct CallFunctionsService {
    repository: Arc<RwLock<EthersRepository>>
}

impl CallFunctionsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>) -> Self{
        CallFunctionsService { repository}
    }

    async fn get_call_response(
        contract: ContractInstance<Arc<Provider<Ws>>, Provider<Ws>>,
        function_name: &str,
    ) -> Option<Token> {
        if let Ok(method) = contract.method(&function_name, ()) {
            method.call().await.ok()
        } else {
            None
        }
    }

    pub async fn exec(&self, user_id:i32, contract_adress:String, functions_name:Vec<String>, abi:String) -> HashMap<String, Value> {

        let provider = {
            let lock = self.repository.read().await;
            lock.get_connection(user_id).expect("ERR CONN")
        };

        let parsed_abi:Abi = serde_json::from_str(&abi).expect("ERR");
        let token_address:Address = contract_adress.parse().unwrap();

        let contract = Contract::new(token_address,parsed_abi,provider);
        let mut functions_response:HashMap<String,Value> = HashMap::new();

        for function_name in functions_name {

            let call_response = Self::get_call_response(contract.clone(), &function_name).await;

            if let Some(token) = call_response {
                let token_to_json = EthersUtils::token_to_json(token);
                functions_response.insert(function_name,token_to_json);
            }

        }

        functions_response

    }

}
