use std::sync::mpsc::Sender;

pub enum MetricType {
    Timer,
    Gauge,
    Counter,
}

pub struct Metric {
    pub key: String,
    pub value: f64,
    pub _type: MetricType
}

pub struct Msg {
    pub payload: String,
}

impl Msg {
    pub fn new(payload: &str) -> Self {
        Msg { payload: payload.to_owned() } 
    }
}

impl Metric {
    pub fn new(key: &str, value: f64, _type: MetricType) -> Self {
        Metric { key: key.to_owned(), value: value, _type: _type }
    }
}

pub trait Consumer {
    fn new(address: &str, bind: bool) -> Self;
    fn subscribe(&mut self, tx: Sender<Msg>, mon: Sender<Metric>);
}
