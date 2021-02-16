use std::mem::{self};
use std::sync::mpsc::{self, Sender};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use mem::MaybeUninit;

// Global data
static mut GLOBAL_DATA: MaybeUninit<GlobalData> = MaybeUninit::uninit();
struct GlobalData {
    pub join_handle: JoinHandle<()>,
    pub sender: Mutex<Sender<BenchMessage>>,
}

// message that will be sent to the event loop
#[derive(Debug)]
pub enum BenchMessage {
    Close,
}

// preiod of event loop update
const EVENT_LOOP_PERIOD: Duration = Duration::from_millis(100);

// method for event loop starting
pub fn instantiate() {
    unsafe {
        // data initialization
        let (tx, rx) = mpsc::channel();

        // wrapping sender into a mutex
        let sender = Mutex::new(tx);

        let join_handle = thread::spawn(move || {
            // write header
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
                    }
                }

                // close if said to do so
                if closed {
                    break 'event_loop;
                }

                thread::sleep(EVENT_LOOP_PERIOD);
            }
            // write footer
        });

        // writing data to global
        GLOBAL_DATA = MaybeUninit::new(GlobalData {
            sender,
            join_handle,
        });
    }
}

// send BenchMessage to the event loop
pub fn send(message: BenchMessage) {
    unsafe {
        println!("Locking");
        let lock = (&*GLOBAL_DATA.as_ptr()).sender.lock().unwrap();
        println!("Locked");
        lock.send(message).unwrap();
    }
}

// deinstantiate the event loop
pub fn deinstantiate() {
    unsafe {
        // send for lock
        send(BenchMessage::Close);

        // getting global data for dropping
        let GlobalData {
            join_handle,
            sender,
        } = mem::replace(&mut GLOBAL_DATA, MaybeUninit::uninit()).assume_init();

        // joinning threads
        join_handle.join().unwrap();

        // dropping the mutex
        drop(sender);
    }
}
