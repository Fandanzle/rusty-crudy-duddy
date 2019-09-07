#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

extern crate uuid;
extern crate serde;
extern crate dotenv;
extern crate rusty_cruddy_simple;

mod lib;
mod schema;
mod message;

use message::{ MessageMap, Message };
use rocket::State;
use rocket_contrib::json::{ JsonValue, Json };
use serde::{Serialize, Deserialize};

use uuid::Uuid;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, Message> = {
        let mut map = HashMap::new();
        map.insert("item", Message{ id: 111, contents: String::from("item")} );
        map
    };
    pub static ref COUNT: usize = HASHMAP.len();
}

#[derive(Serialize, Deserialize)]
pub struct Response<'a> {
    pub id: i32,
    pub contents: &'a Message
}

#[get("/")]
fn index() -> String {
    format!("Hello root, {}!", Uuid::new_v4())
}

#[get("/sample")]
fn sample() -> String {
    format!("Hello sample, {}!", Uuid::new_v4())
}

#[get("/static_map")]
fn static_map() -> Json<&'static Message> {
    let item = HASHMAP.get(&"item").unwrap();
    let item = Response {
        id: 1222,
        contents: item
    };
    return Json(item);
}

#[get("/static_map/count")]
fn static_map_count() -> String {
    let item = *COUNT;
    format!("Hello sample count, {}!", item.to_string())
}

#[get("/create")]
fn new(map: State<'_, MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    let uuid = Uuid::new_v4();
    let message = Message {
        id: 11,
        contents: uuid.to_string()
    };
    hashmap.insert(uuid.to_string(), message);
    println!("{}", uuid.to_string());
    json!({ "status": "ok", "uuid": uuid.to_string() })
}

fn main() {
    lib::establish_connection();
    println!("Running server on port 3000");
    rocket::ignite().mount("/", routes![index, sample, static_map, static_map_count, new]).launch();
}
