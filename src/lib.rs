//! This crate provides the tools to benchmark code for
//! further analyzation using Chrome tracing.
//!
//! # Examples
//!
//! Examples of using gbench basic functionality
//! ```rust
//! use gbench::{instantiate, scope};
//!
//! fn main() {
//!     // Istantiation of the global variables
//!     // It is needed at the top of every program that uses gbench
//!     // The folder that is specified is the folder where the data
//!     // will be saved
//!     instantiate!("target/bench");
//!     {
//!         // This macro creates a variable that starts benchmarking
//!         // on creation and saves the result on drop.
//!         // The variable name is the first argument, the scope name
//!         // is the second.
//!         scope!(sc | "Scope");
//!
//!         for _ in 0..1_000_000 {
//!             let _a = 1 + 1;
//!         }
//!     }
//! }
//! ```
//!
//! Example of a [log!] macro use
//! ```rust
//! use gbench::{instantiate, log, scope};
//!
//! fn main() {
//!     instantiate!(ginst | "target/bench");
//!     {
//!         scope!(sc | "Scope");
//!
//!         for _ in 0..1_000 {
//!             let a = 1 + 1;
//!
//!             // You can log to the file with a timestamp
//!             // using log! macro
//!             log!("A = {}", a);
//!         }
//!     }
//! }
//! ```
//!
//! Example of a [count!] macro use
//! ```rust
//! use gbench::{count, instantiate, scope};
//!
//! fn main() {
//!     instantiate!(ginst | "target/bench");
//!     {
//!         scope!(sc | "Scope");
//!
//!         for i in 0..1000 {
//!             let val = i * i;
//!
//!             // This statement writes val to field "val" of counter "Actual value"
//!             // and writes i to field "i" of counter "I"
//!             count! {
//!                 "Actual value" => {
//!                     "val" => val
//!                 },
//!                 "I" => {
//!                     "i" => i
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Full example
//! ```rust
//! use std::thread;
//!
//! use gbench::{instantiate, scope};
//!
//! fn calculate(num: f32, n: u32) -> f32 {
//!     (0..n)
//!         .fold((num, 0.0), |(x, v), _| (x + v * 0.01, v - x * 0.001))
//!         .0
//! }
//!
//! fn main() {
//!     instantiate!("target/bench");
//!
//!     scope!(program_scope | "Program scope");
//!
//!     // Doing the work that needs benchmarking
//!     for _ in 0..5 {
//!         scope!(main | "Main scope");
//!
//!         // Spawning a thread to do work
//!         let thread = thread::spawn(move || {
//!             // This benchmarks the scope that it is in
//!             scope!(child | "Child");
//!
//!             calculate(1.0, 1_500_000)
//!         });
//!
//!         // You can organize your subtasks in scopes to
//!         // benchmark them
//!         scope!(imp | "An important task");
//!
//!         {
//!             scope!(i1 | "Important subtask");
//!             calculate(1.0, 300_000);
//!         }
//!
//!         {
//!             scope!(i2 | "Less important subtask");
//!             calculate(1.0, 500_000);
//!         }
//!
//!         // If the block of code that you need to benchmark
//!         // has ended you can drop the guard if the scope
//!         // has not ended
//!         drop(imp);
//!
//!         // Marking the start of another task
//!         scope!(join | "Joining thread");
//!
//!         thread.join().unwrap();
//!
//!         // This line of code is unnecessary but I like
//!         // to keep it
//!         drop(join);
//!     }
//! }
//! ```
//!
//! [log!]: macro.log.html
//! [count!]: macro.count.html

mod bench;
mod global;
mod id;

pub use bench::Instantiator;
pub use bench::TimeScope;

#[doc(hidden)]
pub use bench::_log;

#[doc(hidden)]
pub use bench::_count;

/// Benchmarks a scope of code
///
/// # Implementation
///
/// The macro expands into a [TimeScope] declaration
///
/// ```rust
/// scope!(main)
/// // expands into this
/// let main = TimeScope::new(format!("main"));
/// ```
///
/// ```rust
/// scope!(main | "A {}", 0)
/// // expands into this
/// let main = TimeScope::new(format!("A {}", 0));
/// ```
///
/// [TimeScope]: struct.TimeScope.html
///
/// # Examples
///
/// ```rust
/// // You can organize your subtasks in scopes to
/// // benchmark them
/// scope!(imp | "An important task");
///
/// {
///     scope!(i1 | "Important subtask");
///     do_work();
/// }
///
/// {
///     scope!(i2 | "Less important subtask");
///     do_other_work();
/// }
///
/// // If the block of code that you need to benchmark
/// // has ended you can drop the guard if the scope
/// // has not ended
/// drop(imp);
///
/// // rest of the scope...
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

/// Instantiates the global variables for benchmark logging
///
/// This macro should be used at the top of any program using this crate.
///
/// ```rust
/// instantiate!("target/bench");
/// // expands into this
/// let __gbench_instantiator__ = Instantiator::new("target/bench");
/// ```
///
/// ```rust
/// instantiate!(ginst | "target/bench");
/// // expands into this
/// let ginst = Instnatiator::new("target/bench");
/// ```
/// The second option is used when you need to use [end] on the instance.
///
/// [end]: struct.Instantiator.html#method.end
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! instantiate {
    ($name: ident | $folder:expr) => {
        let mut $name = {
            use gbench::Instantiator;
            Instantiator::new($folder)
        };
    };

    ($folder:expr) => {
        instantiate!(__global_instantiator__ | $folder);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! instantiate {
    ($folder: expr) => {};
}

/// Logs data to a benchmarking file
///
/// ```rust
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

/// Creates a counting event
///
/// The following code will log value "i" to a field "val"
/// in counter "CA"
/// ```rust
/// count!("CA" => {"val" => i});
/// ```
///
/// This code will write i to field "a" and i/2 to field "b" in
/// counter "a" and i%3 in field "c" in counter "b".
/// ```rust
/// count!(
///     "a" => {
///         "a" => i,
///         "b" => i / 2
///     },
///     "b" => {
///         "c" => i % 3
///     }
/// );
/// ```
/// For additional information on counter events visit
/// [official chrome tracing documentation]
///
/// [official chrome tracing documentation]:https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/preview#heading=h.msg3086636uq
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! count {
    ($($name: expr => {$($argname: expr => $val:expr),*}),*) => {{
        use gbench::_count as count;

        $(
            let cname = String::from($name);

            let mut data = Vec::new();

            $(
                data.push((String::from($name), $val as f32));
            )*

            count(cname, data);
        )*
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! count {
    ($name: ident => {$($argname: ident => $val: expr),*}) => {};
}
