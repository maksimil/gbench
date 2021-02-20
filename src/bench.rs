use std::mem;
use std::time::Instant;

use crate::global::{begin, begin_time, end, queue_mutex, BenchData};

fn ts_of(instant: Instant) -> u128 {
    instant.duration_since(begin_time()).as_micros()
}

pub fn _log(log: String) {
    let ts = ts_of(Instant::now());

    queue_mutex().push(BenchData::Log { log, ts });
}

/// A function for saving benchmarking results
///
/// ```
/// let start = Instant::now();
/// thread::sleep(Duration::from_millis(500));
/// bench("500ms", start);
/// ```
/// will write this to the benchmarking file
/// ```
/// {
///   "cat": "function",
///   "dur": /* duration of the event */,
///   "name": "500ms",
///   "ph": "X",
///   "pid": 0,
///   "tid": 0,
///   "ts": /* timestamp of start */
/// }
/// ```
pub fn bench(name: String, start: Instant) {
    let ts = ts_of(start);
    let dur = start.elapsed().as_micros();

    queue_mutex().push(BenchData::Bench { name, ts, dur });
}

/// A sctruct used for benchmarking scopes that it is in.
///
/// TimeScope saves the Instant it was created. When dropped it
/// calls [bench] on the instant and a name that was specified
/// in the constructor.
///
/// Using [scope!] macro instead of this struct is recommened.
///
/// [bench]: fn.bench.html
/// [scope!]: macro.scope.html
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
pub struct Instantiator(&'static str);

impl Instantiator {
    pub fn new(folder: &'static str) -> Instantiator {
        begin();
        Instantiator(folder)
    }
}

impl Drop for Instantiator {
    fn drop(&mut self) {
        end(self.0);
    }
}
