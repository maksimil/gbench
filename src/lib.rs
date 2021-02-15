use std::fs::File;
use std::io::Write;
use std::mem::{self, MaybeUninit};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum BenchMessage {
    Close,
    Log { log: String, ts: u128 },
    Bench { name: String, ts: u128, dur: u128 },
}

static mut EVENT_QUEUE: MaybeUninit<Mutex<Vec<BenchMessage>>> = MaybeUninit::uninit();
static mut EVENT_JOIN_HANDLE: Option<JoinHandle<()>> = None;
static mut GBENCH_START: MaybeUninit<Instant> = MaybeUninit::uninit();

pub fn timestamp() -> u128 {
    unsafe { (&*GBENCH_START.as_ptr()).elapsed().as_micros() }
}

pub fn ts_of(instant: &Instant) -> u128 {
    unsafe {
        instant
            .duration_since(*GBENCH_START.as_ptr().clone())
            .as_micros()
    }
}

pub fn begin_gbench(filename: &'static str, period: Duration) {
    unsafe {
        GBENCH_START = MaybeUninit::new(Instant::now());
        EVENT_QUEUE = MaybeUninit::new(Mutex::new(Vec::new()));

        EVENT_JOIN_HANDLE = Some(thread::spawn(move || {
            // open file
            let mut file = File::create(filename).unwrap();

            // write header
            write!(file, "{{\"otherData\":{{}},\"traceEvents\":[{{\"cat\":\"log\",\"name\":\"start\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":{}}}", timestamp()).unwrap();

            // write messages
            'event_loop: loop {
                let queue = {
                    let mut lock = (&*EVENT_QUEUE.as_ptr()).lock().unwrap();
                    mem::replace(&mut *lock, Vec::new())
                };

                let mut closed = false;

                for msg in queue {
                    match msg {
                        BenchMessage::Close => {
                            closed = true;
                        }

                        BenchMessage::Log { log, ts } => {
                            write!(file, ",{{\"cat\":\"log\",\"name\":\"{}\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":{}}}", log, ts).unwrap();
                        }

                        BenchMessage::Bench { name, ts, dur } => {
                            write!(file,",{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":0,\"ts\":\"{}\"}}", dur, name, ts).unwrap();
                        }
                    }
                }

                if closed {
                    break 'event_loop;
                }

                thread::sleep(period);
            }

            // write footer
            write!(file, "]}}").unwrap();
        }));
    }
}

pub fn end_gbench() {
    unsafe {
        send(BenchMessage::Close);

        let join_handle = mem::replace(&mut EVENT_JOIN_HANDLE, None).unwrap();

        join_handle.join().unwrap();
        let _queue = mem::replace(&mut EVENT_QUEUE, MaybeUninit::uninit()).assume_init();
    }
}

pub(crate) fn send(msg: BenchMessage) {
    unsafe {
        let mut queue_lock = (&*EVENT_QUEUE.as_ptr()).lock().unwrap();
        queue_lock.push(msg);
    }
}

pub fn log(log: String) {
    send(BenchMessage::Log {
        log,
        ts: timestamp(),
    });
}

pub fn bench(name: String, start: &Instant) {
    send(BenchMessage::Bench {
        name,
        ts: ts_of(start),
        dur: timestamp() - ts_of(start),
    })
}

#[cfg(test)]
pub mod test;
