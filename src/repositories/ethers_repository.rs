use ethers::providers::{Provider, Ws};
use std::collections::HashMap;
use std::sync::Arc;

pub struct EthersRepository {
    connections: HashMap<i32, Arc<Provider<Ws>>>
}

impl EthersRepository {

    pub fn new() -> Self {
        EthersRepository {
            connections: HashMap::new()
        }
    }

    pub fn apply_connection(&mut self, user_id:i32,provider:Provider<Ws>) {
        let provider = Arc::new(provider);
        self.connections.insert(user_id,provider);
    }

    pub fn get_connection(&self, user_id:i32) -> Option<Arc<Provider<Ws>>> {
        self.connections.get(&user_id).cloned()
    }

}