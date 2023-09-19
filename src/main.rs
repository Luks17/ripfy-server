pub mod conf;
pub mod db;

use conf::Config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONF: Config = Config::new();
}

#[tokio::main]
async fn main() {
    let _db = db::connect().await;
}
