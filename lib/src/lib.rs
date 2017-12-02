#![crate_type = "dylib"]

extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate elastic_types_derive;
extern crate elastic_types;

#[macro_use]
extern crate log;

extern crate json_str;

pub mod models;
pub mod consumer;

use serde_json::{Result, Error};


use models::LogEntry;
use consumer::Msg;

#[no_mangle]
pub fn msg_to_logentry(msg: &Msg) -> Option<LogEntry> {
    match serde_json::from_str(&msg.payload) {
        Ok(msg) => {
            Some(msg)
        },
        Err(error) => {
            error!("Failed to serialize '{}' into Message Object, reason: '{}'", &msg.payload, error);
            None
        }
    }
}

#[no_mangle]
pub fn logentry_to_msg(log: &LogEntry) -> Msg {
    Msg::new(&serde_json::to_string(&log).unwrap())
}
