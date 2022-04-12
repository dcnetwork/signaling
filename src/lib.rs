use mongodb::{bson::doc, options::ClientOptions,Collection, Client};
//
pub mod call;
//
pub static CLIENT_OPT = ClientOptions::parse("mongodb+srv://dcsignal:dcsignal%40n1@cluster0.l8joh.mongodb.net/SigBuffer?retryWrites=true&w=majority",).await.unwrap();

