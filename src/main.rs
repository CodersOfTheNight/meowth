extern crate zmq;
extern crate statsd;
extern crate yaml_rust;
extern crate getopts;
extern crate elastic;

#[macro_use]
extern crate json_str;
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate time;
#[macro_use]
extern crate chrono;

use statsd::Client;
use getopts::Options;
use std::env;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use time::now;
use std::time::Duration;

mod config;
mod models;

use config::Cfg;
use models::LogEntry;

type Msg = String;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} --config cfg", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "config", "Set config name", "config.yaml");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    match matches.opt_str("c") {
      Some(x) => run(&x),
      None => println!("No config file.")
    }

}

fn subscriber(config_obj: &Cfg, tx: Sender<Msg>) {
    info!("Starting subscriber");
    let mut statsd = Client::new(&config_obj.statsd.address, &config_obj.statsd.prefix).unwrap();
    let ctx = zmq::Context::new();

    let mut socket = ctx.socket(zmq::SUB).unwrap();
    socket.bind(&config_obj.zmq.address).unwrap();
    let subscribtion = format!("").into_bytes();
    socket.set_subscribe(&subscribtion).unwrap();

    loop {
        let data = socket.recv_msg(0).unwrap();
        let len = data.len() as f64;
        statsd.incr("messages.unprocessed");
        statsd.gauge("messages.size", len);
        let payload = std::str::from_utf8(&data).unwrap();
        debug!("Payload: {}", payload);
        tx.send(payload.to_owned()).unwrap();
    }
}

fn check_elastic(es: &elastic::prelude::Client) -> bool {
    let ping = elastic::prelude::PingRequest::new();
    let response = es.request(ping).send();
    match response {
        Ok(_) => true,
        Err(_) => false
    }
}

fn worker(config_obj: &Cfg, rx: Receiver<Msg>) {
    info!("Starting worker");
    let url = &config_obj.es.address;
    let params = elastic::prelude::RequestParams::new(url.to_owned());
    let es = elastic::prelude::Client::new(params).unwrap();
    let bulk_size = config_obj.es.bulk_size;

    let sleep_millis = Duration::from_millis(5000);

    let mut statsd = Client::new(&config_obj.statsd.address, &config_obj.statsd.prefix).unwrap();
    loop {
        let date = now();
        let index_str = format!("{0}-{1}-{2}-{3}", &config_obj.es.prefix, (date.tm_year + 1900), date.tm_mon, date.tm_mday);
        debug!("Index is: {}", &index_str);
        let index = elastic::prelude::Index::from(index_str.to_owned());
        let mut messages_pack = String::new();
        let mut pipe = statsd.pipeline();

        for i in 1 .. bulk_size {
            let data = rx.recv().unwrap();
            let msg: LogEntry = serde_json::from_str(&data).unwrap();
            debug!("LogEntry: {}", msg);
            debug!("{} items to go before flush", (bulk_size - i));
            let payload = serde_json::to_string(&msg).unwrap().to_owned();
            messages_pack.push_str(&payload);
            pipe.decr("messages.unprocessed");
        }

        let bulk = elastic::prelude::BulkRequest::for_index(index, messages_pack);
        while !check_elastic(&es) {
            info!("ElasticSearch unreachable. Waiting");
            thread::sleep(sleep_millis);
        }
        pipe.send(&mut statsd);
    }
}

fn run(cfg: &str) {
    env_logger::init().unwrap();
    let config_obj = config::load(cfg).unwrap();

    let (tx, rx): (Sender<Msg>, Receiver<Msg>) = mpsc::channel();

    let config = config_obj.clone();
    let sub = thread::spawn(move || {
        let thread_tx = tx.clone();
        subscriber(&config, thread_tx);
    });


    let config = config_obj.clone();
    thread::spawn(move || {
        worker(&config, rx);
    });

    let result = sub.join();
    assert!(result.is_err());
}

