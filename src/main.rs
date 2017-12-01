extern crate statsd;
extern crate yaml_rust;
extern crate getopts;
extern crate elastic;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate time;

extern crate hostname;

extern crate meowth_lib;

use statsd::Client;
use getopts::Options;
use std::env;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use time::now;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::str;
use hostname::get_hostname;

use meowth_lib::models::LogEntry;
use meowth_lib::consumer::{Msg, Metric, MetricType};
use meowth_lib::{logentry_to_msg, msg_to_logentry};

#[cfg(feature = "zmq")]
extern crate zmq_consumer;

mod config;

mod es_manager;
mod es_client;

use es_client::SyncESClient;
use config::Cfg;
use es_manager::ESManager;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} --config cfg", program);
    print!("{}", opts.usage(&brief));
}

fn get_hash(item: &str) -> String{
    let mut s = DefaultHasher::new();
    item.hash(&mut s);
    String::from(s.finish().to_string())
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

/*fn zmq_subscriber(config_obj: &Cfg, tx: Sender<Msg>) {
    let mut statsd = Client::new(&config_obj.statsd.address, &config_obj.statsd.prefix).unwrap();
    match config_obj.zmq {
        Some(ref cfg) => {
            let ctx = zmq::Context::new();

            let socket = ctx.socket(zmq::PULL).unwrap();
            if cfg.bind {
                socket.bind(&cfg.address).unwrap();
            }
            else {
                socket.connect(&cfg.address).unwrap();
            }

            info!("Starting zMQ subscriber");

            loop {
                let data = socket.recv_msg(0).unwrap();
                let len = data.len() as f64;
                statsd.incr("messages.unprocessed");
                statsd.gauge("messages.size", len);
                let payload = std::str::from_utf8(&data).unwrap();
                trace!("Payload: {}", payload);
                tx.send(payload.to_owned()).unwrap();
            }
        },
        None => {
        }
    }
}*/

fn handle_stream(stream: &mut TcpStream, tx: &Sender<Msg>) {
    let mut buffer: Vec<u8> = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();

    let payload = str::from_utf8(&buffer).unwrap();
    info!("Got tcp data: {}", payload);
}

fn tcp_subscriber(config_obj: &Cfg, tx: Sender<Msg>) {
    let mut statsd = Client::new(&config_obj.statsd.address, &config_obj.statsd.prefix).unwrap();
    match config_obj.tcp {
        Some(ref cfg) => {
            info!("Starting TCP subscriber");
            let addr: &str = &cfg.address;
            let listener = TcpListener::bind(&addr).unwrap();

            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => {
                        handle_stream(&mut s, &tx);
                    },
                    Err(e) => {
                        panic!(e);
                    }
                }
            }
        },
        None => {
        }
    }
}

fn worker(config_obj: &Cfg, rx: Receiver<Msg>, mon: Sender<Metric>) {
    info!("Starting worker");
    let urls = &config_obj.es.address;
    let ty = &config_obj.es.ty;
    let bulk_size = config_obj.es.bulk_size;

    //let mut statsd = Client::new(&config_obj.statsd.address, &config_obj.statsd.prefix).unwrap();
    let mut es: ESManager<SyncESClient> = ESManager::new(urls.clone());
    loop {

        let date = now();
        let index_str = format!("{0}-{1}.{2:02}.{3}", &config_obj.es.prefix, (date.tm_year + 1900), (date.tm_mon + 1), date.tm_mday);
        debug!("Index is: {}", &index_str);
        let mut messages_pack: Vec<String> = Vec::new();
        //let mut pipe = statsd.pipeline();

        for i in 1 .. bulk_size {
            let data = rx.recv().unwrap();
            debug!("Received message: '{}'", &data.payload);
            let mut msg: LogEntry = msg_to_logentry(&data);
            //Extend model with additional fields
            match msg.ty {
                None => {
                    msg.ty = Some(String::from(ty.to_owned()));
                },
                _ => {}
            }

            match msg.host {
                None => {
                    msg.host = get_hostname();
                },
                _ => {}
            }
            msg.ts = Some(msg.time);


            debug!("{} items to go before flush", (bulk_size - i));
            let msg = logentry_to_msg(&msg);
            trace!("Output payload: {}", msg.payload);
            let doc_index = format!("{{\"index\":{{\"_id\":\"{0}\", \"_type\": \"{1}\"}}}}", get_hash(&msg.payload.to_owned()), &ty);
            messages_pack.push(doc_index);
            messages_pack.push(msg.payload);
            mon.send(Metric::new("messages.unprocessed", -1.0, MetricType::Counter)).unwrap();
        }

        es.push_messages(&index_str, &messages_pack);
        es.update();

        //pipe.send(&mut statsd);
    }
}


#[cfg(feature = "zmq")]
fn subscribe(config_obj: &Cfg, tx: Sender<Msg>, mon: Sender<Metric>) {
    use zmq_consumer::ZmqConsumer;
    match config_obj.zmq {
        Some(ref cfg) => {
            let consumer = ZmqConsumer(&cfg.address, cfg.bind);
            let thread_tx = tx.clone();
            let thread_mon = mon.clone();
            thread::spawn(move || {
                consumer.subscribe(thread_tx, thread_mon);
            });
        },
        None => {
        }
    }
}

#[cfg(feature = "tcp")]
fn subscribe(config_obj: &Cfg, tx: Sender<Msg>, mon: Sender<Metric>) {
    match config_obj.tcp {
        Some(ref cfg) => {
            info!("Starting TCP subscriber");
            let addr: &str = &cfg.address;
            let listener = TcpListener::bind(&addr).unwrap();

            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => {
                        handle_stream(&mut s, &tx);
                    },
                    Err(e) => {
                        panic!(e);
                    }
                }
            }
        },
        None => {
        }
    }
}

fn run(cfg: &str) {
    env_logger::init().unwrap();
    let config_obj = config::load(cfg).unwrap();

    let (tx, rx): (Sender<Msg>, Receiver<Msg>) = mpsc::channel();
    let (mon, consume): (Sender<Metric>, Receiver<Metric>) = mpsc::channel();

    let config = config_obj.clone();
    let thread_tx = tx.clone();
    let thread_mon = mon.clone();
    thread::spawn(move || {
        subscribe(&config, thread_tx, thread_mon);
    });

    let config = config_obj.clone();
    let thread_mon = mon.clone();
    let w = thread::spawn(move || {
        worker(&config, rx, thread_mon);
    });

    let result = w.join();
    assert!(result.is_err());
}

