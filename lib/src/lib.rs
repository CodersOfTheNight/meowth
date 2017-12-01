#![crate_type = "dylib"]

extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

//extern crate elastic;
#[macro_use]
extern crate elastic_types_derive;
extern crate elastic_types;

extern crate json_str;

pub mod models;
pub mod consumer;

use models::LogEntry;
use consumer::Msg;

#[no_mangle]
pub fn msg_to_logentry(msg: &Msg) -> LogEntry {
    serde_json::from_str(&msg.payload).unwrap()
}

#[no_mangle]
pub fn logentry_to_msg(log: &LogEntry) -> Msg {
    Msg::new(&serde_json::to_string(&log).unwrap())
}
