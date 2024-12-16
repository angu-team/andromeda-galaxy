use ethers::providers::{Provider, Ws};
use std::collections::HashMap;
pub struct EthersRepository {
    connections: HashMap<i32, Provider<Ws>>
}

impl EthersRepository {

    pub fn new() -> Self {
        EthersRepository {
            connections: HashMap::new()
        }
    }

    pub fn apply_connection(&mut self, user_id:i32,provider:Provider<Ws>) {
        self.connections.insert(user_id,provider);
    }

    pub fn get_connection(&self, user_id:i32) -> Option<&Provider<Ws>> {
        self.connections.get(&user_id)
    }

}