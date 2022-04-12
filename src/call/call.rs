use mongodb::{bson::doc, options::ClientOptions,Collection, Client};
use mongodb::bson::{Bson, oid::ObjectId};
use std::time::Duration;
use std::option::Option;
use tokio::spawn;
use serde::*;
use mongodb;
use hex;
use crate::*;

pub fn start_signal(address:Vec<u8>){
    
    let client_addr = hex::encode(address);
    
    thread::spawn(move || {

        println!("\x1b[1m\x1b[28m[\x1b[35mSIGNAL\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> Started Listening on address ...");
        
        let buffer = CLIENT_OPT.database("SigBuffer").collection::<CallText>("buffer");
        
        while true {
            let timeout = Duration::from_secs(90);
            // take a break of 90s and then continue
            thread::sleep(timeout);
            // check whether there is a call for you
            let res = buffer.find_one( doc! {
                "to": client_addr
            },None).await;
            // match the result
            match res {
                Some(doc) =>{
                    println!("\x1b[1m\x1b[28m[\x1b[35mSIGNAL\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> GOT A MESSAGE FROM ");
                },
                None =>{
                    println!("");
                }
            }
        }
    });
}

// list of database call types and function
#[derive(Serialize, Deserialize, Debug)]
pub struct CallText{

	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
    // address of callee
	pub to: String,
    // address of caller
	pub from: String, 
    // public key of the caller
	pub pubkey: String,
	// #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	//released: chrono::DateTime<Utc>,
}

// push a call onto the buffer
pub async fn push_text_call( doc: &Collection<CallText> , to: String, from: String, pubkey: String ) -> bool {
	// put in the credentials of your and the target	
	let call_this_person = CallText {
        id: None,
        to: "Captain America".to_owned(),
        from: "Thor".to_owned(),
        pubkey: "0xjskdfsjdhfisdhidcsidhfiusdhisdh".to_owned(),
    };
    //
    let serialized = bson::to_bson(&call_this_person).unwrap();
    // convert to document
    let document = serialized.as_document().unwrap();
    // 
    let insert_result = doc.insert_one(document.clone(), None).await.expect("unable to insert document");
    // after a duration of 180 sec delete the call (hang up)

    thread::spawn(move ||{

        let buffer = CLIENT_OPT.database("SigBuffer").collection::<CallText>("buffer");
        let timeout = Duration::from_secs(180);
        thread::sleep(timeout);
        buffer.delete_one( doc! {
            "from": from
        },None).await;
    });
    true
}
