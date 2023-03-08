#[cfg(debug_assertions)]
pub(crate) use log::debug;

#[cfg(debug_assertions)]
pub(crate) use log::trace;

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($arg:tt)+) => {
        // Debug logging disabled in `release` profile
        // Effectively a noop - result ignored by rustc in `release` profile
        format_args!($($arg)+)
    };
}

#[cfg(not(debug_assertions))]
pub(crate) use debug;

#[cfg(not(debug_assertions))]
macro_rules! trace {
    ($($arg:tt)+) => {
        // Trace logging disabled in `release` profile
        // Effectively a noop - result ignored by rustc in `release` profile
        format_args!($($arg)+)
    };
}

#[cfg(not(debug_assertions))]
pub(crate) use trace;
