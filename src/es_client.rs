// Based on: https://github.com/elastic-rs/elastic/blob/master/examples/account_sample/src/ops/mod.rs
use elastic::prelude::*;
use elastic::client::{SyncClientBuilder, SyncClient};

pub trait ESClient {
    fn new(url: &String) -> Self;
    fn push_messages(&mut self, index_str: &String, data: &Vec<String>) -> bool;
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

    fn push_messages(&mut self, index_str: &String, data: &Vec<String>) -> bool {
        let payload = data.join("\n");
        let index = Index::from(index_str.to_owned());
        let bulk = BulkRequest::for_index(index, payload);
        match self.io.request(bulk).send(){
                Ok(_) => {
                    true
                },
                Err(error) => {
                    info!("Failed to push bulk to ElasticSearch. Reason: {:?}. Will retry with next push", error);
                    false
                }
        }
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

