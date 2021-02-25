use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;

use crate::global::BenchData;

pub trait Writer {
    fn end(&self, data: &Vec<BenchData>);
}

pub struct ChromeTracing(pub &'static str);

fn write_data(file: &mut File, data: &BenchData) {
    match data {
        BenchData::Log { log, ts, tid } => write!(
            file,
            "{{\"cat\":\"log\",\"name\":\"{}\",\"ph\":\"I\",\"pid\":0,\"tid\":{},\"ts\":{}}}",
            log, tid, ts
        )
        .unwrap(),
        BenchData::Bench { name, ts, dur, tid } => write!(
            file,
            "{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":{},\"ts\":{}}}", 
            dur, name, tid,  ts
        ).unwrap(),
        BenchData::Count {name, ts, tid, data} => {
            write!(
                file, 
                "{{\"cat\":\"count\",\"name\":\"{}\",\"ph\":\"C\",\"pid\":0,\"tid\":{},\"ts\":{}, \"args\":{{", 
                name, tid, ts
            ).unwrap();

            let mut dataiter = data.into_iter();

            if let Some((name, value)) = dataiter.next() {
                write!(
                    file,
                    "\"{}\":{}",
                    name, value
                ).unwrap();
            }

            for (name, value) in dataiter {
                write!(
                    file, 
                    ",\"{}\":{}",
                    name, value
                ).unwrap();
            }

            write!(
                file, 
                "}}}}"
            ).unwrap();
        }
    }
}

impl Writer for ChromeTracing {
    fn end(&self, data: &Vec<BenchData>) {
        // write data to file
        let folder = self.0;
        let mut file = File::create(format!(
            "{}/bench-{}.json",
            folder,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ))
        .unwrap();

        write!(file, "{{\"otherData\":{{}},\"traceEvents\":[").unwrap();

        let mut data = data.iter();

        // body
        if let Some(data) = data.next() {
            write_data(&mut file, data);
        }

        for data in data {
            write!(file, ",").unwrap();
            write_data(&mut file, data);
        }

        // write footer
        write!(file, "]}}").unwrap();
    }
}
