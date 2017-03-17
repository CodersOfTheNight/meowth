use elastic::prelude::*;
use std::marker::PhantomData;

use std::time::Duration;
use std::thread;

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

fn check_client(es: &Client) -> bool {
    let ping = PingRequest::new();
    let response = es.request(ping).send();
    match response {
        Ok(_) => true,
        Err(_) => false
    }
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

impl<'a> ESManager<'a, Client> {
    pub fn new(urls: Vec<String>) -> ESManager<'a, Client> {
        let size = urls.len() as u32;
        let clients = urls.into_iter().map(|u| {
            let params = RequestParams::new(u.to_owned());
            Client::new(params).unwrap()
        }).collect();

        info!("Creating ElastiSearch Manager with {} clients", size);
        ESManager{ clients: clients, phantom: PhantomData, cursor: RRCursor::new(size as u32) }
    }

    pub fn request<T>(&'a self, req: T) -> RequestBuilder<'a, T>
        where T: Into<HttpRequest<'static>>{
        let index = self.cursor.current() as usize;
        self.clients[index].request(req)
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
}
