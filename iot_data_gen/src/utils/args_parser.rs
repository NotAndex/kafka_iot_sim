use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long = "device_struct")]
    device_struct: String,
    #[arg(short, long = "topic")]
    topic: String,
    #[arg(short, long = "bootstrap_server")]
    bootstrap_server: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Properties {
    pub obs_frequency: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sensor {
    pub name: String,
    pub properties: Properties,
}

pub fn get_args() -> (Vec<Sensor>, String, String) {
    let args = Args::parse();
    let device_struct: Vec<Sensor> = serde_json::from_str(&args.device_struct).unwrap();
    let topic: String = args.topic;
    let bootstrap_server: String = args.bootstrap_server;
    (device_struct, topic, bootstrap_server)
}
