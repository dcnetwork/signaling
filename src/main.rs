use mongodb::{bson::doc, options::ClientOptions, Client};
use signaling::call::dbcall::*;
use mongodb::bson;

#[tokio::main]

async fn main() -> mongodb::error::Result<()> {
    
    let client_options = ClientOptions::parse(
        "mongodb+srv://dcsignal:dcsignal%40n1@cluster0.l8joh.mongodb.net/SigBuffer?retryWrites=true&w=majority",
    ).await.unwrap();

    let client = Client::with_options(client_options).unwrap();

    let buffer = client.database("SigBuffer").collection::<CallText>("buffer");

    // println!("Databases:");
    let call_this_person = CallText {
        id: None,
        to: "Captain America".to_owned(),
        from: "Thor".to_owned(),
        pubkey: "0xjskdfsjdhfisdhidcsidhfiusdhisdh".to_owned(),
    };

    let serialized_movie = bson::to_bson(&call_this_person).unwrap();

    let document = serialized_movie.as_document().unwrap();
    buffer.delete_one( doc! {

      "from": "Thor"

   },None).await;

    // let _new_doc = doc! {
    //     "Address": "0x000"
    // };
    
    // let insert_result = buffer.insert_one(document.clone(), None).await.unwrap();

    // println!("New document ID: {}", insert_result.inserted_id);
    
   //  for name in client.list_database_names(None, None).await.unwrap() {
   //    println!("- {}", name);
   //  }
    Ok(())
}
