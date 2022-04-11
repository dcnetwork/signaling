use mongodb::{bson::doc, options::ClientOptions,Collection, Client};
use serde::*;
use std::option::Option;
use mongodb::bson::{Bson, oid::ObjectId};
use tokio::spawn;
// list of database call types and function

#[derive(Serialize, Deserialize, Debug)]
pub struct CallText{

	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id:    Option<ObjectId>,

	pub to:    String, // address of callee
	pub from:  String, // address of caller
	pub pubkey:String, // public key of the caller

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
    let serialized_movie = bson::to_bson(&call_this_person).unwrap();
    // convert to document
    let document = serialized_movie.as_document().unwrap();
    // 
    let insert_result = doc.insert_one(document.clone(), None).await.unwrap();
    // after a duration of 180 sec delete the call (hang up)


	true
}
