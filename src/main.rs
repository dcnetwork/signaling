use mongodb::{bson::doc, options::ClientOptions, Client};
#[tokio::main]

async fn main() -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse(
        "mongodb+srv://dcsignal:dcsignal%40n1@cluster0.l8joh.mongodb.net/SigBuffer?retryWrites=true&w=majority",
    ).await.unwrap();

    let client = Client::with_options(client_options).unwrap();

    let buffer = client.database("SigBuffer").collection("buffer");

    // println!("Databases:");

    let new_doc = doc! {
        "Address": "0x000"
    };
    
    let insert_result = buffer.insert_one(new_doc.clone(), None).await.unwrap();

    println!("New document ID: {}", insert_result.inserted_id);
    
   //  for name in client.list_database_names(None, None).await.unwrap() {
   //    println!("- {}", name);
   //  }
    Ok(())
}
