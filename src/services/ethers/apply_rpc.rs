use crate::repositories::ethers_repository::Connections;
use ethers::prelude::{Provider, Ws};
use ethers::providers::Middleware;

pub async fn apply_rpc(user_id:i32,endpoint:String) -> Result<(),Err()>{
    let mut conn =  Connections::new();

    let provider = Provider::<Ws>::connect(endpoint).await.expect("ERRCON 500");
    conn.add(user_id, provider);

    Ok(())
}

