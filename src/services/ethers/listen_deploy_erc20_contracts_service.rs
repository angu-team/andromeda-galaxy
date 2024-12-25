use std::sync::{Arc, RwLock};
use ethers::addressbook::Address;
use ethers::prelude::{BlockNumber, StreamExt};
use ethers::providers::Middleware;
use ethers::types::{BlockId, H160};
use ethers::utils::keccak256;
use crate::repositories::ethers_repository::EthersRepository;

pub struct ListenDeployErc20ContractsService {
    repository: Arc<RwLock<EthersRepository>>
}

impl ListenDeployErc20ContractsService {
    pub fn new(repository: Arc<RwLock<EthersRepository>>) -> Self{
        ListenDeployErc20ContractsService { repository}
    }

    pub async fn exec(&self, user_id:i32, webhook:String){
        let lock_provider = Arc::new(self.repository.read().unwrap());

        // Armazenando a conexão em uma variável separada para garantir sua vida útil
        let provider = lock_provider.clone().get_connection(user_id).unwrap();
        let stream = provider.subscribe_blocks().await.unwrap();

        // stream.for_each(|block| {
        //     let provider = lock_provider.clone().get_connection(user_id).unwrap();
        //
        //     // Agora, movemos o block para a closure assíncrona
        //     async move {
        //         let block_id = BlockId::Number(BlockNumber::Number(block.number.unwrap()));
        //         let block_data = provider.get_block_with_txs(block_id).await.expect("ERR").unwrap();
        //         let transactions = block_data.transactions;
        //
        //         for transaction in transactions {
        //             match transaction.to {
        //                 None => {
        //                     let selectors = vec![
        //                         keccak256("totalSupply()")[..4].to_vec(),
        //                         keccak256("balanceOf(address)")[..4].to_vec(),
        //                         keccak256("transfer(address,uint256)")[..4].to_vec(),
        //                         keccak256("approve(address,uint256)")[..4].to_vec(),
        //                         keccak256("allowance(address,address)")[..4].to_vec(),
        //                     ];
        //
        //                     let trimmed_input = &transaction.input[2..]; // Pegue a string sem o "0x"
        //                     let code_bytes = hex::decode(trimmed_input).expect("Bytecode inválido");
        //                     let is_erc20 = selectors.iter().all(|selector| code_bytes.windows(4).any(|window| window == selector.as_slice()));
        //                     println!("{:?} para {:?}",is_erc20,transaction.hash);
        //                 },
        //                 _ => {}
        //             }
        //         }
        //
        //         // println!("Block number: {}", transactions);
        //         // provider.some_method();  // Exemplo de uso do provider
        //     }
        // }).await;

    }

}