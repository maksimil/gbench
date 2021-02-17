use std::io::Write;
use std::time::Instant;

use crate::{begin_time, deinstantiate, file_mutex, instantiate};

fn ts_of(instant: Instant) -> u128 {
    instant.duration_since(begin_time()).as_micros()
}

pub fn log(log: &str) {
    let ts = ts_of(Instant::now());

    let mut file = file_mutex();
    write!(
        file,
        ",{{\"cat\":\"log\",\"name\":\"{}\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":{}}}",
        log, ts
    )
    .unwrap();
}

pub fn bench(name: &str, start: Instant) {
    let ts = ts_of(start);
    let dur = start.elapsed().as_micros();

    let mut file = file_mutex();
    write!(file,
        ",{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":0,\"ts\":{}}}", dur, name, ts
    ).unwrap();
}

pub struct TimeScope {
    start: Instant,
    name: String,
}

impl TimeScope {
    pub fn new(name: String) -> TimeScope {
        TimeScope {
            start: Instant::now(),
            name,
        }
    }
}

impl Drop for TimeScope {
    fn drop(&mut self) {
        bench(&self.name, self.start);
    }
}

pub struct Instantiator;

impl Instantiator {
    pub fn new(folder: &str) -> Instantiator {
        instantiate(folder);
        Instantiator
    }
}

impl Drop for Instantiator {
    fn drop(&mut self) {
        deinstantiate();
    }
}
