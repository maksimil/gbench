use std::collections::HashSet;
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

pub struct CsvWriter(pub &'static str);

const DELIMITER: char = ';';

fn tstr(v: f32) -> String {
    v.to_string().replace(".", ",")
}

impl Writer for CsvWriter {
    fn end(&self, data: &Vec<BenchData>) {
        let folder= self.0;
        let mut file = File::create(format!(
            "{}/graph-{}.csv", 
            folder,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis())
        ).unwrap();

        let (fields, csvdata) = {
            let mut fields = HashSet::new();
            let mut csvdata = Vec::new();

            for bdata in data {
                if let BenchData::Count {name, ts, data, tid: _} = bdata {
                    for (varname, value) in data {
                        let fieldname = format!("{} : {}", name, varname);
                        csvdata.push((fieldname.clone(), *value, *ts));
                        fields.insert(fieldname);
                    }
                }
            }

            let mut fields = fields.into_iter().collect::<Vec<_>>();
            fields.sort();

            let csvdata = csvdata.into_iter().map(|(fieldname, value, ts)| {
                (fields.binary_search(&fieldname).unwrap(), value, ts)
            }).collect::<Vec<_>>();

            (fields, csvdata)
        };

        let fieldcount = fields.len();
        let rows = csvdata.into_iter().scan(vec![None;fieldcount], |state, (idx, value, ts)| {
            state[idx] = Some(value);
            Some((ts, state.clone()))
        });

        write!(file, "ts").unwrap();

        for field in fields {
            write!(file, "{}{}", DELIMITER, field).unwrap();
        }

        write!(file, "\n").unwrap();
        
        for (ts, data) in rows {
            write!(file, "{}", tstr(ts)).unwrap();
            
            for datapart in data {
                write!(file, "{}", DELIMITER).unwrap();
                if let Some(data) = datapart {
                    write!(file, "{}", tstr(data)).unwrap();
                }
            }
            
            write!(file, "\n").unwrap();
        }
    }
}