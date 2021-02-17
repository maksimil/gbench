mod bench;
mod global;

pub use global::deinstantiate;
pub use global::instantiate;

pub use global::begin_time;
pub use global::file_mutex;

pub use bench::bench;
pub use bench::log;
pub use bench::TimeScope;

#[macro_export]
macro_rules! scope {
    ($name: ident) => {
        let $name = TimeScope::new(String::from(stringify!($name)));
    };
}
