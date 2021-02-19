use std::io::Write;
use std::time::Instant;

use crate::{begin, begin_time, end, file_mutex};

fn ts_of(instant: Instant) -> u128 {
    instant.duration_since(begin_time()).as_micros()
}

pub fn _log(log: &str) {
    let ts = ts_of(Instant::now());

    let mut file = file_mutex();
    write!(
        file,
        ",{{\"cat\":\"log\",\"name\":\"{}\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":{}}}",
        log, ts
    )
    .unwrap();
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
pub fn bench(name: &str, start: Instant) {
    let ts = ts_of(start);
    let dur = start.elapsed().as_micros();

    let mut file = file_mutex();
    write!(file,
        ",{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":0,\"ts\":{}}}", dur, name, ts
    ).unwrap();
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
pub struct TimeScope<S: AsRef<str>> {
    start: Instant,
    name: S,
}

impl<S: AsRef<str>> TimeScope<S> {
    pub fn new(name: S) -> TimeScope<S> {
        TimeScope {
            start: Instant::now(),
            name,
        }
    }
}

impl<S: AsRef<str>> Drop for TimeScope<S> {
    fn drop(&mut self) {
        bench(self.name.as_ref(), self.start);
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
pub struct Instantiator;

impl Instantiator {
    pub fn new(folder: &str) -> Instantiator {
        begin(folder);
        Instantiator
    }
}

impl Drop for Instantiator {
    fn drop(&mut self) {
        end();
    }
}
