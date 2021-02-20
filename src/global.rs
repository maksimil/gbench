use std::fs::File;
use std::io::Write;
use std::mem::{self, MaybeUninit};
use std::sync::{Mutex, MutexGuard};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// Global data
pub enum BenchData {
    Log { log: String, ts: u128 },
    Bench { name: String, ts: u128, dur: u128 },
}

static mut GLOBAL_DATA: MaybeUninit<GlobalData> = MaybeUninit::uninit();
struct GlobalData {
    pub program_begin: Instant,
    pub queue_mutex: Mutex<Vec<BenchData>>,
}

// method for event loop starting
pub fn begin() {
    // data initialization
    let program_begin = Instant::now();

    // file mutex
    let queue_mutex = Mutex::new(Vec::new());

    // writing data to global
    unsafe {
        GLOBAL_DATA = MaybeUninit::new(GlobalData {
            queue_mutex,
            program_begin,
        });
    }
}

// time of program beginning
pub fn begin_time() -> Instant {
    unsafe { (&*GLOBAL_DATA.as_ptr()).program_begin }
}

// getting queue mutex
pub fn queue_mutex() -> MutexGuard<'static, Vec<BenchData>> {
    unsafe { (&*GLOBAL_DATA.as_ptr()).queue_mutex.lock().unwrap() }
}

fn write_data(file: &mut File, data: BenchData) {
    match data {
        BenchData::Log { log, ts } => write!(
            file,
            "{{\"cat\":\"log\",\"name\":\"{}\",\"ph\":\"I\",\"pid\":0,\"tid\":0,\"ts\":{}}}",
            log, ts
        )
        .unwrap(),
        BenchData::Bench { name, ts, dur } => write!(file,
            "{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":0,\"ts\":{}}}", dur, name, ts
        ).unwrap(),
    }
}

// deinstantiate the event loop
pub fn end(folder: &str) {
    // get data to write
    let mut data = {
        let mut lock = queue_mutex();
        mem::replace(&mut *lock, Vec::new())
    }
    .into_iter();

    // write data to file
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

    // getting global data for dropping
    unsafe {
        let _gd = mem::replace(&mut GLOBAL_DATA, MaybeUninit::uninit()).assume_init();
    }
}
