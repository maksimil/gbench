use std::mem::{self, MaybeUninit};
use std::sync::{Mutex, MutexGuard};
use std::time::Instant;

use crate::id::{IdBorrow, IdStorage};
use crate::writer::Writer;

// Global data

/// Enum that represents different benchmarking data
///
/// Data that is passed to the writers is in form of these enums.
///
/// # Fields
/// - **ts** -  timestamp
/// - **dur** - duration
/// - **tid** - thread id
#[derive(Debug, Clone)]
pub enum BenchData {
    /// Log contains logging data produced by the [log!](macro.log.html) macro
    Log { log: String, ts: f32, tid: usize },

    /// Bench contains benchmarking data produced by the [scope!](macro.scope.html) macro
    Bench {
        name: String,
        ts: f32,
        dur: f32,
        tid: usize,
    },

    /// Count contains counting data produced by the [count!](macro.count.html) macro
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

// deinstantiate the gobal data
pub fn end(writers: Vec<Box<dyn Writer + 'static>>) {
    // get data to write
    let data = {
        let mut lock = queue_mutex();
        mem::replace(&mut *lock, Vec::new())
    };

    // write data to writers
    for writer in writers {
        writer.end(&data);
    }

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
