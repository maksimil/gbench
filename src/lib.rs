mod bench;
mod global;

pub(crate) use global::begin_time;
pub(crate) use global::deinstantiate;
pub(crate) use global::file_mutex;
pub(crate) use global::instantiate;

pub use bench::bench;
pub use bench::log;
pub use bench::Instantiator;
pub use bench::TimeScope;

#[macro_export]
macro_rules! scope {
    ($name: ident) => {
        use gbench::TimeScope;
        let $name = TimeScope::new(stringify!($name));
    };
}

#[macro_export]
macro_rules! instantiate {
    ($folder:expr) => {
        use gbench::Instantiator;
        let __instantiator__ = Instantiator::new($folder);
    };
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        use gbench::log;
        let log_string = format!($($arg)*);
        log(&log_string);
    };
}
