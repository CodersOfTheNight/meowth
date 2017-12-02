use std::marker::PhantomData;
use std::time::Duration;
use std::thread;


use es_client::ESClient;

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
    retry_queue: Vec<(String, Vec<String>)>
}

impl<'a, TClient: ESClient> ESManager<'a, TClient>
{
    pub fn new(urls: Vec<String>) -> ESManager<'a, TClient> {
        let size = urls.len() as u32;
        let clients = urls.into_iter().map(|u| {
            TClient::new(&u)
        }).collect();

        info!("Creating ElastiSearch Manager with {} clients", size);
        ESManager{ clients: clients, phantom: PhantomData, cursor: RRCursor::new(size as u32), retry_queue: Vec::new() }
    }

    pub fn push_messages(&mut self, index_str: &String, data: &Vec<String>) -> bool {
        let index = self.cursor.current() as usize;
        if !self.clients[index].push_messages(index_str, data) {
            self.retry_queue.push((index_str.clone(), data.clone()));
            false
        }
        else{
            true
        }
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

        // Handle Retry Queue
        while let Some(head) = self.retry_queue.pop() {
            let (index, data) = head;
            self.push_messages(&index, &data);
        }
    }
}
