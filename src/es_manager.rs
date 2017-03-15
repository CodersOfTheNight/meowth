use elastic::prelude::*;
use std::marker::PhantomData;


pub struct ESManager<'a, T: 'a> {
    clients: Vec<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a> ESManager<'a, Client> {
    pub fn new(urls: Vec<String>) -> ESManager<'a, Client> {
        let clients = urls.into_iter().map(|u| {
            let params = RequestParams::new(u.to_owned());
            Client::new(params).unwrap()
        }).collect();
        ESManager{ clients: clients, phantom: PhantomData }
    }

    pub fn request<T>(&'a self, req: T) -> RequestBuilder<'a, T>
        where T: Into<HttpRequest<'static>>{
        self.clients[0].request(req)
    }

    fn check_client(es: &Client) -> bool {
        let ping = PingRequest::new();
        let response = es.request(ping).send();
        match response {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
