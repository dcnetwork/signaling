use mongodb::{bson::doc, options::ClientOptions, Client};
use mongodb::bson;
use signaling::call::*;
#[tokio::main]

async fn main() -> mongodb::error::Result<()> {
    
    push_text_call("Thor".to_string(),"Captain America".to_string(),"thisismypubkey".to_string()).await;
    start_signal(vec![0]).await;
    Ok(())
}
