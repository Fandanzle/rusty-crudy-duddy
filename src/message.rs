use std::sync::Mutex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// The type to represent the ID of a message.
pub type ID = usize;

// We're going to store all of the messages here. No need for a DB.
pub type MessageMap = Mutex<HashMap<String, Message>>;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub contents: String
}
