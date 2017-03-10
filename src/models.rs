
use elastic::prelude::*;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
pub struct LogEntry {
    process: i32,
    channel: String,
    func_name: String,
    args: Vec<String>,
    file_name: String,
    module: String,
    host: String,
    message: String,
    lineno: i32,
    timestamp: Date<DefaultDateFormat>
}
