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

#[cfg(debug_assertions)]
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

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! scope {
    ($name:ident) => {};

    ($name:ident|$($arg:tt)*) => {};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! instantiate {
    ($folder:expr) => {
        use gbench::Instantiator;
        let __gbench_instantiator__ = Instantiator::new($folder);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! instantiate {
    ($folder: expr) => {};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        use gbench::_log as log;
        let log_string = format!($($arg)*);
        log(&log_string);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {};
}
