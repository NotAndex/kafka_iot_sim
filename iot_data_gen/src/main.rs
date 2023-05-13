#![allow(unused_imports, unused)]
use chrono::Utc;
use rand::Rng;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};
use serde_json;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread;

mod utils;
use utils::args_parser::get_args;
use utils::time::sleep_until_next_interval;

fn observation(sensor_name: String) -> HashMap<String, String> {
    let timestamp: String = Utc::now().to_string();
    let value: String = rand::thread_rng().gen::<f64>().to_string();
    let mut observation: HashMap<String, String> = HashMap::new();
    observation.insert(String::from("timestamp"), timestamp);
    observation.insert(String::from("value"), value);
    let mut message: HashMap<String, String> = HashMap::new();
    message.insert(String::from("sensor_name"), sensor_name);
    message.insert(String::from("observation"), serde_json::to_string(&observation).unwrap());

    message
}

fn sensor(sender: Sender<HashMap<String, String>>, sensor_name: String, obs_frequency: u64) {
    loop {
        sender.send(observation(sensor_name.clone())).unwrap();
        sleep_until_next_interval(obs_frequency as u128)
    }
}

fn main() {
    let (device_struct, topic, bootstrap_server) = get_args();
    let sensor_count = device_struct.len();

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .create()
        .expect("Producer creation error");

    //create senders and one receiver
    let (tx, rx) = channel::<HashMap<String, String>>();
    let mut txs: Vec<Sender<HashMap<String, String>>> = Vec::new();
    for _ in 0..(sensor_count - 1) {
        let tx_clone: Sender<HashMap<String, String>> = tx.clone();
        txs.push(tx_clone);
    }
    txs.push(tx);

    sleep_until_next_interval(1);

    // creating thread for each sensor
    for (sensor_obj, t) in device_struct.into_iter().zip(txs.into_iter()) {
        thread::spawn(move || sensor(t, sensor_obj.name, sensor_obj.properties.obs_frequency.parse().unwrap_or(5)));
    }
    let mut msg_batches = Vec::new();

    for received in rx {
        msg_batches.push(received);
        if msg_batches.len() == 10 {
            println!("┌{:─<108}┐", "");
            for msg in &msg_batches {
                let key: &String = msg.get("sensor_name").unwrap();
                let payload: &String = msg.get("observation").unwrap();
                println!("│ key: {} ║ value -> {}{}", key, format!("{:<82}", payload), "│");

                producer
                    .send(BaseRecord::to(&topic).payload(payload).key(key))
                    .expect("Failed to enqueue");
            }
            println!("└{:─<108}┘", "");
            msg_batches.clear();
        }
    }
}
