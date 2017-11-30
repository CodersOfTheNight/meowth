use elastic::prelude::*;
use elastic::client::requests::RequestBuilder;
use elastic::client::Sender;

use std::marker::PhantomData;
use std::time::Duration;
use std::thread;


use es_client::ESClient;


macro_rules! ensure {
    ($($body: expr), *) => {

        let sleep_millis = Duration::from_millis(5000);

        loop {
            let response = $($body)*;
            match response {
                Ok(_) => {
                    break
                },
                Err(error) => {
                        info!("Failed to push bulk to ElasticSearch. Reason: {:?}. Retrying", error);
                        thread::sleep(sleep_millis);
                        continue
                },
            }
        };
    };
}

fn check_client<TClient: ESClient>(es: &mut TClient) -> bool {
    es.ping()
}



trait Cursor {
    fn current(&self) -> u32;
    fn next(&mut self);
}


#[derive(Clone)]
struct RRCursor {
    current: u32,
    end: u32
}

impl RRCursor {
    fn new(end: u32) -> RRCursor {
        RRCursor { current: 0, end: end }
    }
}

impl Cursor for RRCursor {
    fn current(&self) -> u32 {
        self.current
    }

    fn next(&mut self){
        if self.current < (self.end - 1) {
            self.current += 1;
        }
        else {
            self.current = 0;
        }
    }
}

#[derive(Clone)]
pub struct ESManager<'a, T: 'a> {
    clients: Vec<T>,
    cursor: RRCursor,
    phantom: PhantomData<&'a T>,
}

impl<'a, TClient: ESClient> ESManager<'a, TClient>
{
    pub fn new(urls: Vec<String>) -> ESManager<'a, TClient> {
        let size = urls.len() as u32;
        let clients = urls.into_iter().map(|u| {
            TClient::new(&u)
        }).collect();

        info!("Creating ElastiSearch Manager with {} clients", size);
        ESManager{ clients: clients, phantom: PhantomData, cursor: RRCursor::new(size as u32) }
    }

    pub fn push_messages(&mut self, data: &Vec<String>) -> bool {
        let index = self.cursor.current() as usize;
        self.clients[index].push_messages(data)
    }

    pub fn update(&mut self) {
        self.cursor.next();
        let mut fail_count = 0;

        let sleep_millis = Duration::from_millis(15000);

        loop {
            let index = self.cursor.current() as usize;
            let client = &mut self.clients[index];
            if !check_client(client) {
                info!("Client unreachable. Fetching next one");
                self.cursor.next();
                fail_count += 1;
            }
            else {
                break;
            }

            if fail_count > self.cursor.end {
                info!("Non of the clients was able to connect. Sleeping for 15sec");
                thread::sleep(sleep_millis);
                fail_count = 0;
            }
        }
    }

/*
    pub fn create_bulk<T>(index_str: &String, payload: T) -> BulkRequest<'a, T> {
        let index = Index::from(index_str.to_owned());
        BulkRequest::for_index(index, payload)
    }
    */
}
