use elastic::prelude::*;
use std::marker::PhantomData;

use std::time::Duration;

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
        if self.current < self.end {
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
        ESManager{ clients: clients, phantom: PhantomData, cursor: RRCursor { end: size as u32, current: 0 } }
    }

    pub fn request<T>(&'a self, req: T) -> RequestBuilder<'a, T>
        where T: Into<HttpRequest<'static>>{
        self.clients[0].request(req)
    }


    pub fn check_connection(self) -> bool {
        self.clients.into_iter().any(|c| {
            check_client(&c)
        })
    }
}
