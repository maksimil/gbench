use std::time::Instant;

use crate::global::{begin_time, send, BenchMessage};

fn ts_of(instant: Instant) -> u128 {
    instant.duration_since(begin_time()).as_micros()
}

pub fn log(log: String) {
    send(BenchMessage::Log {
        log,
        ts: ts_of(Instant::now()),
    });
}
