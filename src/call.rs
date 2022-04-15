use mongodb::{bson::doc,bson, options::ClientOptions,Collection, Client};
use mongodb::bson::{Bson,document::Document, oid::ObjectId};

use std::time::Duration;
use std::option::Option;
use std::thread;
use std::borrow::Borrow;
use tokio::spawn;
use serde::*;
use mongodb;
use hex;
use crate::*;

pub async fn start_signal(address:Vec<u8>){
    
    let client_opt = ClientOptions::parse("mongodb+srv://dcsignal:dcsignal%40n1@cluster0.l8joh.mongodb.net/SigBuffer?retryWrites=true&w=majority",).await.unwrap();
    let client = Client::with_options(client_opt).unwrap();

    let client_addr = "Thor";//hex::encode(address);
    //
    tokio::spawn(async move {
        //
        println!("\x1b[1m\x1b[28m[\x1b[35mSIGNAL\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> Started Listening on address ...");
        //
        let buffer = client.database("SigBuffer").collection::<CallText>("buffer");
        
        loop {
            let timeout = Duration::from_secs(90);
            // take a break of 90s and then continue
            thread::sleep(timeout);
            // check whether there is a call for you
            let res = buffer.find_one( doc! {
                "to": client_addr.clone()
            },None).await.unwrap();
            // match the result
            match res {
                Some(doc) =>{
                    print!("\x1b[1m\x1b[28m[\x1b[35mSIGNAL\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> GOT A MESSAGE FROM ");
                    match doc {
                        CallText { to:a, from:b, pubkey:c,.. } =>{
                            println!("{:?}", b);
                        }
                        _=>{

                        }
                    }                    
                },
                None =>{
                    println!("\x1b[1m\x1b[28m[\x1b[35mSIGNAL\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> NONE & TIMEOUT  ");
                }
            }
        }
    }).await;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CallAck{

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    // address of caller
    pub to: String,
    // address of callee
    pub from: String, 
    // public key of the callee
    pub pubkey: String,
    //
    pub transID: String,
    // #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    //released: chrono::DateTime<Utc>,
}




// push a call onto the buffer
pub async fn push_text_call( to: String, from: String, pubkey: String ) -> bool {
	// put in the credentials of your and the target	
	let call_this_person = CallText {
        id: None,
        to: to.to_owned(),
        from: from.to_owned(),
        pubkey: pubkey.to_owned(),
    };
    //
    let client_opt = ClientOptions::parse("mongodb+srv://dcsignal:dcsignal%40n1@cluster0.l8joh.mongodb.net/SigBuffer?retryWrites=true&w=majority",).await.unwrap();
    let client = Client::with_options(client_opt).unwrap();
    //
    let buffer = client.database("SigBuffer").collection::<Document>("buffer");

    let serialized = bson::to_bson(&call_this_person).unwrap();
    // convert to document
    let document = serialized.as_document().unwrap();
    // 
    let insert_result = buffer.insert_one(document.to_owned(), None).await.expect("unable to insert document");
    // it is just like a call ring ...
    // after a duration of 180 sec delete the call (hang up)
    tokio::spawn(async move {

        let buffer = client.database("SigBuffer").collection::<CallText>("buffer");
        let timeout = Duration::from_secs(180);
        thread::sleep(timeout);
        buffer.delete_one( doc! {
            "from": from
        },None).await;

    });

    true
}
/*
from = your address
check for any ack message assigned by the following address
*/
pub async fn wait_ack_call(from: String,to:String){
    let client_opt = ClientOptions::parse("mongodb+srv://dcsignal:dcsignal%40n1@cluster0.l8joh.mongodb.net/SigBuffer?retryWrites=true&w=majority",).await.unwrap();
    let client = Client::with_options(client_opt).unwrap();
    let buffer = client.database("SigBuffer").collection::<CallAck>("ack");
    tokio::spawn(async move{
        let res = buffer.find_one( doc! {
            "to": from,
            "from": to
        },None).await.unwrap();
        // match the result
        match res {
            Some(doc) =>{
                print!("\x1b[1m\x1b[28m[\x1b[35mSIGNAL\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> GOT A MESSAGE FROM ");
                match doc {
                    CallAck { to:a, from:b, pubkey:c,.. } =>{
                        println!("{:?}", b);
                    }
                    _=>{

                    }
                }                    
            },
            None =>{
                println!("\x1b[1m\x1b[28m[\x1b[35mSIGNAL\x1b[0m\x1b[1m\x1b[28m]\x1b[0m ~> NONE & TIMEOUT  ");
            }
        }

    });
}
