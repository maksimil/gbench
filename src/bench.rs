use std::mem;
use std::time::Instant;

use crate::global::{begin, begin_time, end, gen_id, get_id, queue_mutex, BenchData};

fn ts_of(instant: Instant) -> f32 {
    (instant.duration_since(begin_time()).as_nanos() as f32) / 1000.0
}

fn enqueue(data: BenchData) {
    let mut lock = queue_mutex();
    lock.push(data);
}

pub fn _log(log: String) {
    let ts = ts_of(Instant::now());
    let tid = get_id();

    enqueue(BenchData::Log { log, ts, tid });
}

fn bench(name: String, ts: f32) {
    let dur = ts_of(Instant::now()) - ts;
    let tid = get_id();

    enqueue(BenchData::Bench { name, ts, dur, tid });
}

pub fn _count(name: String, data: Vec<(String, f32)>) {
    let ts = ts_of(Instant::now());
    let tid = get_id();

    enqueue(BenchData::Count {
        name,
        data,
        ts,
        tid,
    });
}

/// A sctruct used for benchmarking scopes that it is in.
///
/// TimeScope saves the Instant it was created. When dropped it
/// saves the benchmarking results to the file.
///
/// Using [scope!] macro instead of this struct is recommened.
///
/// [scope!]: macro.scope.html
pub struct TimeScope {
    start: f32,
    name: String,
}

impl TimeScope {
    pub fn new(name: String) -> TimeScope {
        gen_id();
        TimeScope {
            start: ts_of(Instant::now()),
            name,
        }
    }
}

impl Drop for TimeScope {
    fn drop(&mut self) {
        bench(mem::replace(&mut self.name, String::new()), self.start);
    }
}

/// A sctruct used for instantiating global data.
///
/// This struct instantiates global data upon creation
/// and deinstantiates it upon drop.
///
/// Using [instantiate!] macro instead of this struct is recommened.
///
/// [instantiate!]: macro.instantiate.html
pub struct Instantiator(&'static str, bool);

impl Instantiator {
    /// Constructs the instantiator.
    pub fn new(folder: &'static str) -> Instantiator {
        begin();
        Instantiator(folder, true)
    }

    /// Deinstantiates global variables.
    ///
    /// This method is used when Instantiator is never dropped.
    pub fn end(&mut self) {
        if self.1 {
            self.1 = false;
            end(self.0);
        }
    }
}

impl Drop for Instantiator {
    fn drop(&mut self) {
        end(self.0);
    }
}
