use deadpool_redis::{Connection, Manager, Pool};

pub struct RedisRepository {
    pool:  Pool
}

impl RedisRepository {
    pub fn connect() -> Self {
        let manager = Manager::new("redis://default:251410@operacional_redis:6379").expect("ERR REDIS");
        let pool = Pool::builder(manager).max_size(16).build().unwrap();

        Self { pool }
    }

    pub async fn get_conn(&self) -> Connection {
        self.pool.get().await.unwrap()
    }

}