use std::fs::read_to_string;
use std::time::Duration;

use crate::{begin_gbench, end_gbench, send, BenchMessage};

#[test]
fn simple_write() {
    begin_gbench("target/bench.json", Duration::from_millis(100));
    send(BenchMessage::Log {
        log: String::from("Log"),
        ts: 0,
    });

    end_gbench();

    let contents = read_to_string("target/bench.json").unwrap();

    assert_eq!(&contents, "{\"otherData\":{},\"traceEvents\":[{\"cat\":\"log\",\"name\":\"start\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":0},{\"cat\":\"log\",\"name\":\"Log\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":0}]}");
}
