// Based on: https://github.com/elastic-rs/elastic/blob/master/examples/account_sample/src/ops/mod.rs
use elastic::prelude::*;
use elastic::client::{SyncClientBuilder, SyncClient};
use elastic::error::Result;

pub trait ESClient {
    fn new(url: &String) -> Self;
    fn push_messages(&mut self, data: &Vec<String>) -> bool;
    fn ping(&mut self) -> bool;
}

pub struct SyncESClient {
    io: SyncClient,
}

impl SyncESClient {}

impl ESClient for SyncESClient {
    fn new(url: &String) -> SyncESClient {
        let client = SyncClientBuilder::new().base_url(url.to_owned()).build().unwrap();

        SyncESClient { io: client }
    }

    fn push_messages(&mut self, data: &Vec<String>) -> bool {
        true
    }

    fn ping(&mut self) -> bool {
        let ping = PingRequest::new();
        let response = self.io.request(ping).send();
        match response {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

