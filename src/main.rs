extern crate zmq;
extern crate statsd;

use statsd::Client;

fn main() {
    let ctx = zmq::Context::new();
    let mut statsd = Client::new("127.0.0.1:8125", "logs").unwrap();

    let mut socket = ctx.socket(zmq::SUB).unwrap();
    socket.bind("tcp://127.0.0.1:50020").unwrap();
    let subscribtion = format!("").into_bytes();
    socket.set_subscribe(&subscribtion).unwrap();

    loop {
        let data = socket.recv_msg(0).unwrap();
        statsd.incr("messages.counter");
        println!("{}", std::str::from_utf8(&data).unwrap());
    }
}
