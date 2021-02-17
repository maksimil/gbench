use std::time::Instant;

use crate::{begin_time, send, BenchMessage};

fn ts_of(instant: Instant) -> u128 {
    instant.duration_since(begin_time()).as_micros()
}

pub fn log(log: String) {
    let ts = ts_of(Instant::now());
    send(BenchMessage::Log { log, ts });
}

pub fn bench(name: String, start: Instant) {
    let ts = ts_of(start);
    let dur = start.elapsed().as_micros();
    send(BenchMessage::Bench { name, ts, dur });
}
