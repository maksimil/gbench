//! This crate provides the tools to benchmark code and then analyzing the
//! results using Chrome tracing.

mod bench;
mod global;

pub use bench::bench;
pub use bench::Instantiator;
pub use bench::TimeScope;

#[doc(hidden)]
pub use bench::_log;

/// A macro for benchmarking a scope of code
///
/// ```
/// scope!(main)
/// // expands into this
/// let main = TimeScope::new("main");
/// ```
///
/// ```
/// scope!(main | "A {}", 0)
/// // expands into this
/// let main = TimeScope::new("A 0");
/// ```
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! scope {
    ($name:ident) => {
        scope!($name | stringify!(name));
    };

    ($name:ident | $($arg:tt)*) => {
        let $name = {
            use gbench::TimeScope;
            TimeScope::new(format!($($arg)*))
        };
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! scope {
    ($name:ident) => {};

    ($name:ident|$($arg:tt)*) => {};
}

/// A macro for instantiating the global environment for benchmark logging.
///
/// This macro should be used at the top of any program using this crate.
///
/// ```
/// instantiate!("target/bench");
/// // expands into this
/// let __gbench_instantiator__ = Instantiator::new("target/bench");
/// ```
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! instantiate {
    ($folder:expr) => {
        let __gbench_instantiator__ = {
            use gbench::Instantiator;
            Instantiator::new($folder)
        };
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! instantiate {
    ($folder: expr) => {};
}

/// A macro for logging an event.
///
/// ```
/// let a = 0;
/// log!("A: {}", a);
/// ```
/// will write this to the benchmarking file
/// ```
/// {
///   "cat": "log",
///   "name": "A: 0",
///   "ph": "I",
///   "pid": 0,
///   "tid": 0,
///   "ts": /* current timestamp */
/// }
/// ```
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        {
            use gbench::_log as log;
            let log_string = format!($($arg)*);
            log(log_string);
        }
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {};
}
