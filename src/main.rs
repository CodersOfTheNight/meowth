extern crate zmq;

fn main() {
    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.bind("tcp://127.0.0.1:50020").unwrap();
    let subscribtion = format!("").into_bytes();
    socket.set_subscribe(&subscribtion).unwrap();

    loop {
        let data = socket.recv_msg(0).unwrap();
        println!("{}", std::str::from_utf8(&data).unwrap());
    }
}
