extern crate meowth_lib;
extern crate zmq;

#[macro_use]
extern crate log;

use meowth_lib::consumer::{Consumer, Msg, MetricType, Metric};
use std::sync::mpsc::Sender;

pub struct ZmqConsumer{
    socket: zmq::Socket,
}


impl Consumer for ZmqConsumer {
    fn new(address: &str, bind: bool) -> Self{
        let ctx = zmq::Context::new();

        let socket = ctx.socket(zmq::PULL).unwrap();
        if bind {
            socket.bind(&address).unwrap();
        }
        else {
            socket.connect(&address).unwrap();
        }
        ZmqConsumer {socket: socket}
    }

    fn subscribe(&mut self, tx: Sender<Msg>, mon: Sender<Metric>) {
        info!("Starting zMQ subscriber");

        loop {
            let data = self.socket.recv_msg(0).unwrap();
            let len = data.len() as f64;
            mon.send(Metric::new("messages.size", len, MetricType::Gauge)).unwrap();
            mon.send(Metric::new("messages.unprocessed", 1.0, MetricType::Counter)).unwrap();
            let payload = std::str::from_utf8(&data).unwrap();
            trace!("Payload: {}", payload);
            tx.send(Msg::new(payload)).unwrap();
        }
    }
}
