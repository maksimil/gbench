use std::fs::File;
use std::io::Write;
use std::mem::{self, MaybeUninit};
use std::sync::{Mutex, MutexGuard};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::id::{IdBorrow, IdStorage};

// Global data
pub enum BenchData {
    Log {
        log: String,
        ts: f32,
        tid: usize,
    },
    Bench {
        name: String,
        ts: f32,
        dur: f32,
        tid: usize,
    },
    Count {
        name: String,
        ts: f32,
        tid: usize,
        data: Vec<(String, f32)>,
    },
}

static mut GLOBAL_DATA: MaybeUninit<GlobalData> = MaybeUninit::uninit();
struct GlobalData {
    pub program_begin: Instant,
    pub queue_mutex: Mutex<Vec<BenchData>>,
    pub id_storage: IdStorage,
}

// method for event loop starting
pub fn begin() {
    // data initialization
    let program_begin = Instant::now();

    // file mutex
    let queue_mutex = Mutex::new(Vec::new());

    // id storage
    let id_storage = IdStorage::new();

    // writing data to global
    unsafe {
        GLOBAL_DATA = MaybeUninit::new(GlobalData {
            queue_mutex,
            program_begin,
            id_storage,
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

thread_local! {
    static TID: IdBorrow = unsafe { (&mut *GLOBAL_DATA.as_mut_ptr()).id_storage.gen() };
}

pub fn gen_id() {
    TID.with(|_| {});
}

pub fn get_id() -> usize {
    TID.with(|tid| tid.id())
}
