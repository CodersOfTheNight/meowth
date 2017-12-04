extern crate meowth_lib;

#[macro_use]
extern crate log;

use meowth_lib::consumer::{Consumer, Msg, MetricType, Metric};
use std::sync::mpsc::Sender;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::str;

pub struct TCPConsumer{
    socket: TcpListener
}


fn handle_stream(stream: &mut TcpStream, tx: &Sender<Msg>, mon: &Sender<Metric>) {
    let mut buf = vec![0u8; 128];
    let mut buffer: Vec<u8> = Vec::new();
    let token: u8 = '\n' as u8;

    loop{
        stream.read_exact(&mut buf).unwrap();
        match buf.iter().position(|&x| x == token) {
            Some(idx) => {
                buffer.extend_from_slice(&buf[1..idx]);
                let mut left_over = &buf[idx..buf.len()];


                let len = buffer.len() as f64;
                mon.send(Metric::new("messages.size", len, MetricType::Gauge)).unwrap();
                mon.send(Metric::new("messages.unprocessed", 1.0, MetricType::Counter)).unwrap();

                let data = buffer.clone();
                let payload = str::from_utf8(&data).unwrap();
                trace!("Payload: {}", payload);
                tx.send(Msg::new(payload)).unwrap();

                buffer.clear();
                buffer.extend_from_slice(&mut left_over);
            },
            None => {
                buffer.extend(&buf.clone());
                trace!("Current buffer {}", str::from_utf8(&buffer).unwrap());
            }
        }
    }

}


impl Consumer for TCPConsumer {
    fn new(address: &str, bind: bool) -> Self{
        if bind {
            let listener = TcpListener::bind(&address).unwrap();
            TCPConsumer {socket: listener}
        }
        else {
            panic!("Only bind is available for TCPConsumer for now");
        }
    }

    fn subscribe(&mut self, tx: Sender<Msg>, mon: Sender<Metric>) {
        info!("Starting TCP subscriber");


        for stream in self.socket.incoming() {
            match stream {
                Ok(mut s) => {
                    handle_stream(&mut s, &tx, &mon);
                },
                Err(e) => {
                    panic!(e);
                }
            }
        }
    }
}
