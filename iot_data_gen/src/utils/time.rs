use std::thread;
use std::time::{Duration, SystemTime};

pub fn sleep_until_next_interval(sec: u128) {
    let duration_since_epoch: Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let timestamp_nanos: u128 = duration_since_epoch.as_nanos(); // u128
    let nanos_until_next_sec: u128 =
        (sec - timestamp_nanos / 1_000_000_000 % sec) * 1_000_000_000 - (timestamp_nanos % 1_000_000_000);
    thread::sleep(Duration::from_nanos(nanos_until_next_sec as u64));
}
