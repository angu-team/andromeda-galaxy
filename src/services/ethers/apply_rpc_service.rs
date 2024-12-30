use crate::repositories::ethers_repository::EthersRepository;
use crate::repositories::redis_repository::RedisRepository;
use ethers::prelude::{Provider, Ws};
use ethers::providers::Middleware;
use redis::AsyncCommands;
use std::sync::{Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ApplyRpcService {
    repository:Arc<RwLock<EthersRepository>>,
    redis_repository:Arc<RedisRepository>,
}

impl ApplyRpcService {
    pub fn new(ethers_repository:Arc<RwLock<EthersRepository>>, redis_repository: Arc<RedisRepository>) -> Self {
        ApplyRpcService {
            repository : ethers_repository,
            redis_repository
        }
    }

    pub async fn exec(&self, user_id:i32, endpoint:String){
        self.start_connection(user_id, endpoint.clone()).await;
        self.start_block_listener(user_id, endpoint.clone()).await;
    }

    async fn start_connection(&self, user_id: i32, endpoint: String){
        let mut redis_conn = self.redis_repository.get_conn().await;

        let provider = Provider::<Ws>::connect(&endpoint).await.expect("ERRCON 500");
        self.repository.write().await.apply_connection(user_id, provider);

        let _: i64 = redis_conn.hset("connections",user_id.to_string(),&endpoint).await.expect("ae");
    }

    async fn start_block_listener(&self, user_id:i32, endpoint: String){
        let provider = Provider::<Ws>::connect(&endpoint).await.expect("ERRCON 500");
        self.repository.write().await.apply_block_listener(user_id, provider).await;
    }

}
