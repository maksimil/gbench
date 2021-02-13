use std::fs::File;
use std::io::Write;
use std::mem::{self, MaybeUninit};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub enum BenchMessage {
    Close,
    Write(String),
}

static mut EVENT_QUEUE: MaybeUninit<Mutex<Vec<BenchMessage>>> = MaybeUninit::uninit();
static mut EVENT_JOIN_HANDLE: Option<JoinHandle<()>> = None;

pub fn send(msg: BenchMessage) {
    unsafe {
        let mut queue_lock = (&*EVENT_QUEUE.as_ptr()).lock().unwrap();
        queue_lock.push(msg);

        println!("{:?}", &*queue_lock);
    }
}

pub fn begin_gbench(filename: &'static str, period: Duration) {
    unsafe {
        println!("Initializing");
        EVENT_QUEUE = MaybeUninit::new(Mutex::new(Vec::new()));

        println!("Initializing EVENT_JOIN_HANDLE");
        EVENT_JOIN_HANDLE = Some(thread::spawn(move || {
            // open file
            println!("Opening fiile");
            let mut file = File::create(filename).unwrap();
            // write header

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

                        BenchMessage::Write(msg) => {
                            println!("{}", msg);
                            write!(file, "{}", msg).unwrap();
                        }
                    }
                }

                if closed {
                    break 'event_loop;
                }

                thread::sleep(period);
            }
            // write footer
        }));

        println!("Initialized {:?}", EVENT_JOIN_HANDLE);
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

#[cfg(test)]
pub mod test;
