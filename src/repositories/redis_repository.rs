use deadpool_redis::{Connection, Manager, Pool};

pub struct RedisRepository {
    pool:  Pool
}

impl RedisRepository {
    pub fn connect(uri:&str) -> Self {
        let manager = Manager::new(uri).expect("ERR REDIS");
        let pool = Pool::builder(manager).max_size(16).build().unwrap();

        Self { pool }
    }

    pub async fn get_conn(&self) -> Connection {
        self.pool.get().await.unwrap()
    }

}