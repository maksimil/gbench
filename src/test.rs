use std::fs::read_to_string;
use std::time::Duration;

use crate::{begin_gbench, end_gbench, send, BenchMessage};

#[test]
fn simple_write() {
    begin_gbench("target/bench.json", Duration::from_millis(100));
    send(BenchMessage::Write(String::from("Ass")));

    end_gbench();

    let contents = read_to_string("target/bench.json").unwrap();

    assert_eq!(&contents, "Ass");
}
