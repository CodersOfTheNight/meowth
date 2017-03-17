use std::io;
use std::fs::File;
use std::io::prelude::*;

use yaml_rust::{YamlLoader, Yaml};

trait Config<T> {
    fn load(doc: &Yaml) -> T;
}

#[derive(Clone)]
pub struct StatsdCfg {
    pub address: String,
    pub prefix: String,
}


#[derive(Clone)]
pub struct ZmqCfg {
    pub address: String,
    pub bind: bool,
}

#[derive(Clone)]
pub struct ElasticCfg {
    pub address: Vec<String>,
    pub prefix: String,
    pub bulk_size: i64,
    pub ty: String,
}


#[derive(Clone)]
pub struct TcpCfg {
    pub address: String,
}

#[derive(Clone)]
pub struct Cfg {
    pub statsd: StatsdCfg,
    pub zmq: Option<ZmqCfg>,
    pub tcp: Option<TcpCfg>,
    pub es: ElasticCfg,
}

impl Config<StatsdCfg> for StatsdCfg {
    fn load(doc: &Yaml) -> StatsdCfg {
        StatsdCfg {
            address: doc["address"].as_str().unwrap().to_owned(),
            prefix: doc["prefix"].as_str().unwrap().to_owned()
        }
    }
}


impl Config<ZmqCfg> for ZmqCfg {
    fn load(doc: &Yaml) -> ZmqCfg {
        ZmqCfg {

            address: doc["address"].as_str().unwrap().to_owned(),
            bind: doc["bind"].as_bool().unwrap(),
        }
    }
}


impl Config<TcpCfg> for TcpCfg {
    fn load(doc: &Yaml) -> TcpCfg {
        TcpCfg {
            address: doc["address"].as_str().unwrap().to_owned(),
        }
    }
}

impl Config<ElasticCfg> for ElasticCfg {
    fn load(doc: &Yaml) -> ElasticCfg {
        let addresses: &Vec<Yaml> = doc["address"].as_vec().unwrap();
        ElasticCfg {
            address: addresses.iter().map(|addr| { addr.as_str().unwrap().to_owned() }).collect(),
            prefix: doc["prefix"].as_str().unwrap().to_owned(),
            bulk_size: doc["bulk_size"].as_i64().unwrap(),
            ty: doc["type"].as_str().unwrap().to_owned(),
        }
    }
}


impl Config<Cfg> for Cfg {
    fn load(doc: &Yaml) -> Cfg {
        let statsd = StatsdCfg::load(&doc["statsd"]);
        let zmq = match doc["zeromq"].is_badvalue() {
            false => {
                Some(ZmqCfg::load(&doc["zeromq"]))
            },
            true => None,
        };

        let tcp = match doc["tcp"].is_badvalue() {
            false => {
                Some(TcpCfg::load(&doc["tcp"]))
            },
            true => None,
        };

        let es = ElasticCfg::load(&doc["elastic_search"]);

        Cfg {
            statsd: statsd,
            zmq: zmq,
            es: es,
            tcp: tcp,
        }
    }
}



fn read(dir: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(dir));
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => return Ok(s),
        Err(e) => return Err(e)
    }
}

pub fn load(dir: &str) -> Result<Cfg, io::Error>{
    let raw = read(dir);
    match raw {
        Ok(s) => {
            let docs = YamlLoader::load_from_str(&s).unwrap();
            let doc = &docs[0];
            return Ok(Cfg::load(&doc))
        },
        Err(e) => return Err(e)

    }
}
