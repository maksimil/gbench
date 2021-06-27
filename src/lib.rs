//! This crate provides the tools to benchmark code for
//! further analyzation using different tools.
//!
//! # Writers System
//!
//! This crate uses writers system to manage collected data.
//! This means that all the macros that collect data during
//! program running push this data in form of [BenchData] enums
//! to a shared storage. At the end of the program all the data is given
//! to the instances of [Writer] that are given at the initialization.
//!
//! # Examples
//!
//! Examples of using gbench basic functionality
//! ```rust
//! use gbench::{instantiate, scope, ChromeTracing};
//! fn main() {
//!     // Istantiation of the global variables
//!     // It is needed at the top of every program that uses gbench
//!     // The folder that is specified is the folder where the data
//!     // will be saved
//!     instantiate!(ChromeTracing("target/bench"));
//!     {
//!         // This macro creates a variable that starts benchmarking
//!         // on creation and saves the result on drop.
//!         // The variable name is the first argument, the scope name
//!         // is the second.
//!         scope!(sc | "Scope");
//!         for _ in 0..1_000_000 {
//!             let _a = 1 + 1;
//!         }
//!     }
//! }
//! ```
//!
//! Example of a [log!] macro use
//! ```rust
//! use gbench::{instantiate, log, scope, ChromeTracing};
//!
//! fn main() {
//!     instantiate!(ChromeTracing("target/bench"));
//!     {
//!         scope!(sc | "Scope");
//!         for _ in 0..1_000 {
//!             let a = 1 + 1;
//!             // You can log to the file with a timestamp
//!             // using log! macro
//!             log!("A = {}", a);
//!         }
//!     }
//! }
//! ```
//!
//! Example of a [count!] macro and [CsvWriter] writer use
//! ```rust
//! use gbench::{count, instantiate, scope, ChromeTracing, CsvWriter};
//!
//! fn main() {
//!     // Additionally CsvWriter will save all the counter data in
//!     // a csv table
//!     instantiate!(ChromeTracing("target/bench"), CsvWriter("target/bench"));
//!     {
//!         scope!(sc | "Scope");
//!         for i in 0..1000 {
//!             let val = i * i;
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
//! use gbench::{instantiate, scope, ChromeTracing};
//! use std::thread;
//! fn calculate(num: f32, n: u32) -> f32 {
//!     (0..n)
//!         .fold((num, 0.0), |(x, v), _| (x + v * 0.01, v - x * 0.001))
//!         .0
//! }
//!
//! fn main() {
//!     instantiate!(ChromeTracing("target/bench"));
//!
//!     scope!(program_scope | "Program scope");
//!     // Doing the work that needs benchmarking
//!     for _ in 0..5 {
//!         scope!(main | "Main scope");
//!         // Spawning a thread to do work
//!         let thread = thread::spawn(move || {
//!             // This benchmarks the scope that it is in
//!             scope!(child | "Child");
//!             calculate(1.0, 1_500_000)
//!         });
//!         // You can organize your subtasks in scopes to
//!         // benchmark them
//!         scope!(imp | "An important task");
//!         {
//!             scope!(i1 | "Important subtask");
//!             calculate(1.0, 300_000);
//!         }
//!         {
//!             scope!(i2 | "Less important subtask");
//!             calculate(1.0, 500_000);
//!         }
//!         // If the block of code that you need to benchmark
//!         // has ended you can drop the guard if the scope
//!         // has not ended
//!         drop(imp);
//!         // Marking the start of another task
//!         scope!(join | "Joining thread");
//!         thread.join().unwrap();
//!         // This line of code is unnecessary but I like
//!         // to keep it
//!         drop(join);
//!     }
//! }
//! ```
//!
//! [log!]: macro.log.html
//! [count!]: macro.count.html
//! [CsvWriter]: struct.CsvWriter.html
//! [BenchData]: enum.BenchData.html
//! [Writer]: trait.Writer.html

mod bench;
mod global;
mod id;
mod writer;

pub use bench::Instantiator;
pub use bench::TimeScope;

pub use global::BenchData;
pub use writer::ChromeTracing;
pub use writer::CsvWriter;
pub use writer::Writer;

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
/// # Implementation
///
/// This macro expands into a declaration of [Instantiator] which instantiates
/// global variables on creation and deinstantiates them on drop.
///
/// ```rust
/// instantiate!(ChromeTracing("target/bench"));
/// // expands into this
/// let __gbench_instantiator__ = Instantiator::new(vec![Box::new(ChromeTracing("target/bench"))]);
/// ```
///
/// If you need to deinstantiate global variables before variable goes out
/// of scope you can specify the variable name and then call [end] on it
/// when you need the deinstantiation.
///
/// ```rust
/// instantiate!(ginst | ChromeTracing("target/bench"));
/// // expands into this
/// let ginst = Instantiator::new(vec![Box::new(ChromeTracing("target/bench"))]);
/// ```
///
/// [Instantiator]: struct.Instantiator.html
/// [end]: struct.Instantiator.html#method.end
///
/// # Examples
/// ```rust
/// use gbench::{instantiate, scope, ChromeTracing};
///
/// fn main() {
///     instantiate!(ChromeTracing("target/bench"));
///     {
///         scope!(sc | "Scope");
///
///         for _ in 0..1_000_000 {
///             let _a = 1 + 1;
///         }
///     }
/// }
/// ```
/// or using [end]
/// ```rust
/// use gbench::{instantiate, scope, ChromeTracing};
///
/// fn main() {
///     instantiate!(ginst | ChromeTracing("target/bench"));
///     {
///         scope!(sc | "Scope");
///
///         for _ in 0..1_000_000 {
///             let _a = 1 + 1;
///         }
///     }
///     ginst.end();
/// }
/// ```
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! instantiate {
    ($name: ident | $($writer: expr),*) => {
        let mut $name = {
            use gbench::Instantiator;

            let mut writers = std::vec::Vec::new();

            $(
                writers.push(std::boxed::Box::new($writer) as std::boxed::Box<dyn gbench::Writer + 'static>);
            )*

            Instantiator::new(writers)
        };
    };

    ($($writer: expr),*) => {
        instantiate!(__global_instantiator__ | $($writer),*);
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
/// will queue this [BenchData]
/// ```
/// Log {
///     log: "A: 0",
///     ts: /* event's timestamp */,
///     tid: /* event's thread of execution */,
/// }
/// ```
///
/// [BenchData]: enum.BenchData.html
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
/// ```rust
/// let i = 10;
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
///
/// Will queue these [BenchData]
///
/// ```
/// Count {
///     name: "a",
///     ts: /* event's timestamp */,
///     tid: /* event's thread of execution */,
///     data: [
///         (
///             "a",
///             10.0,
///         ),
///         (
///             "b",
///             5.0,
///         ),
///     ],
/// },
/// Count {
///     name: "b",
///     ts: /* event's timestamp */,
///     tid: /* event's thread of execution */,
///     data: [
///         (
///             "c",
///             1.0,
///         ),
///     ],
/// },
/// ```
///
/// [BenchData]: enum.BenchData.html
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! count {
    ($($name: expr => {$($argname: expr => $val:expr),*}),*) => {{
        use gbench::_count as count;

        $(
            let cname = std::string::String::from($name);

            let mut data = std::vec::Vec::new();

            $(
                data.push((std::string::String::from($argname), $val as f32));
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
