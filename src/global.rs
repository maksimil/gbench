use std::fs::File;
use std::io::Write;
use std::mem::{self};
use std::sync::mpsc::{self, Sender};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use mem::MaybeUninit;

// Global data
static mut GLOBAL_DATA: MaybeUninit<GlobalData> = MaybeUninit::uninit();
struct GlobalData {
    pub program_begin: Instant,
    pub join_handle: JoinHandle<()>,
    pub sender: Mutex<Sender<BenchMessage>>,
}

// message that will be sent to the event loop
#[derive(Debug)]
pub(crate) enum BenchMessage {
    Close,
    Log { log: String, ts: u128 },
}

// preiod of event loop update
const EVENT_LOOP_PERIOD: Duration = Duration::from_millis(100);

// method for event loop starting
pub fn instantiate() {
    unsafe {
        // data initialization
        let program_begin = Instant::now();
        let (tx, rx) = mpsc::channel();

        // wrapping sender into a mutex
        let sender = Mutex::new(tx);

        let join_handle = thread::spawn(move || {
            // write header
            let mut file = File::create(format!(
                "target/bench-{}.json",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            ))
            .unwrap();

            write!(file, "{{\"otherData\":{{}},\"traceEvents\":[{{\"cat\":\"log\",\"name\":\"start\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":0}}").unwrap();

            // write body
            'event_loop: loop {
                let mut closed = false;

                // run through messages
                for message in rx.try_iter() {
                    println!("Message: {:?}", message);
                    match message {
                        // mark the loop for close if said so
                        BenchMessage::Close => {
                            closed = true;
                        }
                        // write logging data
                        BenchMessage::Log { log, ts } => {
                            write!(file, ",{{\"cat\":\"log\",\"name\":\"{}\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":{}}}", log, ts).unwrap();
                        }
                    }
                }

                // close if said to do so
                if closed {
                    break 'event_loop;
                }

                thread::sleep(EVENT_LOOP_PERIOD);
            }
            // write footer
            write!(file, "]}}").unwrap();
        });

        // writing data to global
        GLOBAL_DATA = MaybeUninit::new(GlobalData {
            sender,
            join_handle,
            program_begin,
        });
    }
}

// send BenchMessage to the event loop
pub(crate) fn send(message: BenchMessage) {
    unsafe {
        println!("Locking");
        let lock = (&*GLOBAL_DATA.as_ptr()).sender.lock().unwrap();
        println!("Locked");
        lock.send(message).unwrap();
    }
}

// time of program beginning
pub(crate) fn begin_time() -> Instant {
    unsafe { (&*GLOBAL_DATA.as_ptr()).program_begin }
}

// deinstantiate the event loop
pub fn deinstantiate() {
    unsafe {
        // send for lock
        send(BenchMessage::Close);

        // getting global data for dropping
        let GlobalData {
            join_handle,
            sender: _sender,
            program_begin: _program_begin,
        } = mem::replace(&mut GLOBAL_DATA, MaybeUninit::uninit()).assume_init();

        // joinning threads
        join_handle.join().unwrap();
    }
}
