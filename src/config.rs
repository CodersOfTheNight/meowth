use std::io;
use std::fs::File;
use std::io::prelude::*;

use yaml_rust::{YamlLoader, Yaml};

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

            let statsd = StatsdCfg {
                address: doc["statsd"]["address"].as_str().unwrap().to_owned(),
                prefix: doc["statsd"]["prefix"].as_str().unwrap().to_owned()
            };

            let zmq = match doc["zeromq"].is_badvalue() {
                false => {
                    let cfg = ZmqCfg {
                        address: doc["zeromq"]["address"].as_str().unwrap().to_owned(),
                        bind: doc["zeromq"]["bind"].as_bool().unwrap(),
                    };
                    Some(cfg)
                },
                true => None,
            };

            let tcp = match doc["tcp"].is_badvalue() {
                false => {
                    let cfg = TcpCfg {
                        address: doc["tcp"]["address"].as_str().unwrap().to_owned(),
                    };
                    Some(cfg)
                },
                true => None,
            };

            let addresses: &Vec<Yaml> = doc["elastic_search"]["address"].as_vec().unwrap();

            let es = ElasticCfg {
                address: addresses.iter().map(|addr| { addr.as_str().unwrap().to_owned() }).collect(),
                prefix: doc["elastic_search"]["prefix"].as_str().unwrap().to_owned(),
                bulk_size: doc["elastic_search"]["bulk_size"].as_i64().unwrap(),
                ty: doc["elastic_search"]["type"].as_str().unwrap().to_owned(),
            };

            let config = Cfg {
                statsd: statsd,
                zmq: zmq,
                es: es,
                tcp: tcp,
            };
            return Ok(config)
        },
        Err(e) => return Err(e)

    }
}
