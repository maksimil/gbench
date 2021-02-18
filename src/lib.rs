mod bench;
mod global;

pub use global::deinstantiate;
pub use global::instantiate;

pub use global::begin_time;
pub use global::file_mutex;

pub use bench::bench;
pub use bench::log;
pub use bench::Instantiator;
pub use bench::TimeScope;

#[macro_export]
macro_rules! scope {
    ($name: ident) => {
        let $name = TimeScope::new(stringify!($name));
    };
}

#[macro_export]
macro_rules! instantiate {
    ($folder:expr) => {
        let __instantiator__ = Instantiator::new($folder);
    };
}
