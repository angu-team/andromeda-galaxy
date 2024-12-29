use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use ethers::abi::Abi;
use ethers::contract::{Contract, ContractError};
use ethers::middleware::Middleware;
use ethers::abi::Token;

use ethers::types::Address;
use serde_json::{json, Value};
use crate::repositories::ethers_repository::EthersRepository;
use crate::utils::abi_utils::AbiUtils;
use crate::utils::ethers_utils::EthersUtils;

pub struct CallFunctionsService {
    repository: Arc<RwLock<EthersRepository>>
}

impl CallFunctionsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>) -> Self{
        CallFunctionsService { repository}
    }

    pub async fn exec(&self, user_id:i32, contract_adress:String, functions_name:Vec<String>) -> HashMap<String, Value> {
        let provider = self.repository.read().unwrap().get_connection(user_id).expect("ERR ");

        let parsed_abi:Abi = serde_json::from_str(AbiUtils::erc20_abi()).expect("ERR");
        let token_address:Address = contract_adress.parse().unwrap();

        let contract = Contract::new(token_address,parsed_abi,provider);
        let mut functions_response:HashMap<String,Value> = HashMap::new();

        for function_name in functions_name {
            let call_response: Token = contract.method(&function_name, ()).expect("ERR CALL METHOD").call().await.expect("ERR CALL METHOD2");
            let token_to_json = EthersUtils::token_to_json(call_response);

            functions_response.insert(function_name,token_to_json);
        }

        functions_response

    }

}
