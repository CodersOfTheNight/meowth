use elastic::prelude::*;
use chrono::prelude::*;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
pub struct LogEntry {
    process: i32,
    channel: String,
    func_name: String,
    args: Vec<String>,
    filename: String,
    module: String,
    host: Option<String>,
    message: String,
    lineno: i32,
    time: DateTime<UTC>,
    level: i32,
    #[serde(rename = "type")] 
    pub ty: Option<String>,
}


impl fmt::Display for LogEntry {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Msg: '{0}' @ {1} line: {2}", &self.message.to_owned(), &self.func_name.to_owned(), &self.lineno.to_owned())

    }
}
