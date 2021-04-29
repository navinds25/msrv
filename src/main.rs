use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::process::exit;
mod webmain;

#[derive(Serialize, Deserialize)]
struct Config {
    address: String,
    media_path: String,
}

gflags::define! {
    -c, --config: &Path
}

fn main() -> std::io::Result<()> {
    gflags::parse();
    let config_filename = CONFIG.flag;
    if !config_filename.exists() {
        println!("can't find config file: {:?}", config_filename.display());
        exit(1);
    }
    let config_file = File::open(config_filename).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Can't parse json config");
    println!("address: {:?}", config.address);
    println!("media_path: {:?}", config.media_path);
    return webmain::startsrv(config.address, config.media_path);
}
