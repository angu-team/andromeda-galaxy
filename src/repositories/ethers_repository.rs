use std::collections::HashMap;
use ethers::providers::{Provider,Ws};
pub struct Connections {
    connections: HashMap<i32, Provider<Ws>>
}

impl Connections {

    pub fn new() -> Self {
        Connections {
            connections: HashMap::new()
        }
    }

    pub fn add(&mut self, user_id:i32,provider:Provider<Ws>) {
        self.connections.insert(user_id,provider);
    }

    pub fn get(&self, user_id:i32) -> Option<&Provider<Ws>> {
        self.connections.get(&user_id)
    }

}