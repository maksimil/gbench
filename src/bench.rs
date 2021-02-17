use std::io::Write;
use std::time::Instant;

use crate::{begin_time, file_mutex};

fn ts_of(instant: Instant) -> u128 {
    instant.duration_since(begin_time()).as_micros()
}

pub fn log(log: String) {
    let ts = ts_of(Instant::now());

    let mut file = file_mutex();
    write!(
        file,
        ",{{\"cat\":\"log\",\"name\":\"{}\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":{}}}",
        log, ts
    )
    .unwrap();
}

pub fn bench(name: String, start: Instant) {
    let ts = ts_of(start);
    let dur = start.elapsed().as_micros();

    let mut file = file_mutex();
    write!(file,
        ",{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":0,\"ts\":{}}}", dur, name, ts
    ).unwrap();
}
