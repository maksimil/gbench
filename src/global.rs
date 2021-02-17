use std::fs::File;
use std::io::Write;
use std::mem;
use std::sync::{Mutex, MutexGuard};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use mem::MaybeUninit;

// Global data
static mut GLOBAL_DATA: MaybeUninit<GlobalData> = MaybeUninit::uninit();
struct GlobalData {
    pub program_begin: Instant,
    pub file_mutex: Mutex<File>,
}

// method for event loop starting
pub fn instantiate(folder: &str) {
    // data initialization
    let program_begin = Instant::now();

    // file mutex
    let file_mutex = Mutex::new({
        let mut file = File::create(format!(
            "{}/bench-{}.json",
            folder,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ))
        .unwrap();

        write!(file, "{{\"otherData\":{{}},\"traceEvents\":[{{\"cat\":\"log\",\"name\":\"start\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":0}}").unwrap();

        file
    });

    // writing data to global
    unsafe {
        GLOBAL_DATA = MaybeUninit::new(GlobalData {
            file_mutex,
            program_begin,
        });
    }
}

// time of program beginning
pub fn begin_time() -> Instant {
    unsafe { (&*GLOBAL_DATA.as_ptr()).program_begin }
}

// file mutex
pub fn file_mutex() -> MutexGuard<'static, File> {
    unsafe { (&*GLOBAL_DATA.as_ptr()).file_mutex.lock().unwrap() }
}

// deinstantiate the event loop
pub fn deinstantiate() {
    unsafe {
        // write footer
        let mut file = file_mutex();
        write!(file, "]}}").unwrap();

        drop(file);

        // getting global data for dropping
        let _gd = mem::replace(&mut GLOBAL_DATA, MaybeUninit::uninit()).assume_init();
    }
}
