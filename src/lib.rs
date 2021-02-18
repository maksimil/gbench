mod bench;
mod global;

pub(crate) use global::begin;
pub(crate) use global::begin_time;
pub(crate) use global::end;
pub(crate) use global::file_mutex;

pub use bench::bench;
pub use bench::Instantiator;
pub use bench::TimeScope;
pub use bench::_log;

#[macro_export]
macro_rules! scope {
    ($name:ident) => {
        scope!($name | stringify!(name));
    };

    ($name:ident | $($arg:tt)*) => {
        use gbench::TimeScope;
        let name = TimeScope::new(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! instantiate {
    ($folder:expr) => {
        use gbench::Instantiator;
        let __gbench_instantiator__ = Instantiator::new($folder);
    };
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        use gbench::_log as log;
        let log_string = format!($($arg)*);
        log(&log_string);
    };
}
