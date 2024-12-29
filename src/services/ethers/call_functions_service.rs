use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use ethers::abi::Abi;
use ethers::contract::{Contract, ContractError};
use ethers::middleware::Middleware;
use ethers::providers::{Provider, Ws};
use ethers::types::Address;
use crate::repositories::ethers_repository::EthersRepository;
use crate::utils::abi_utils::AbiUtils;

pub struct CallFunctionsService {
    repository: Arc<RwLock<EthersRepository>>
}

impl CallFunctionsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>) -> Self{
        CallFunctionsService { repository}
    }

    pub async fn exec(&self, user_id:i32,contract_adress:String,functions_name:Vec<String>) -> HashMap<String,String>{
        let provider = self.repository.read().unwrap().get_connection(user_id).expect("ERR ");

        let parsed_abi:Abi = serde_json::from_str(AbiUtils::erc20_abi()).expect("ERR");
        let token_address:Address = contract_adress.parse().unwrap();

        let contract = Contract::new(token_address,parsed_abi,provider);
        let mut functions_response:HashMap<String,String> = HashMap::new();

        for function_name in functions_name {
            let response: String = contract.method::<_, String>(&function_name, ()).expect("ERR").call().await.expect("ERR");
            functions_response.insert(function_name,response);
        }

        functions_response

    }

}
