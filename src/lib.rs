mod bench;
mod global;

pub use global::deinstantiate;
pub use global::instantiate;

pub use global::begin_time;
pub use global::send;
pub use global::BenchMessage;

pub use bench::bench;
pub use bench::log;
