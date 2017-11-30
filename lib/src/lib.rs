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

use models::LogEntry;

#[no_mangle]
pub fn msg_to_logentry(msg: &String) -> LogEntry {
    serde_json::from_str(&msg).unwrap()
}

#[no_mangle]
pub fn logentry_to_msg(log: &LogEntry) -> String {
    serde_json::to_string(&log).unwrap()
}
