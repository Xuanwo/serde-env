#[cfg(debug_assertions)]
pub(crate) use log::debug;

#[cfg(debug_assertions)]
pub(crate) use log::trace;

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($arg:tt)+) => {
        // Debug logging disabled in `release` profile
    };
}

#[cfg(not(debug_assertions))]
pub(crate) use debug;

#[cfg(not(debug_assertions))]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        $($arg)+
        // Trace logging disabled in `release` profile
    };
    ($($arg:tt)+) => {
        // Trace logging disabled in `release` profile
    };
}

#[cfg(not(debug_assertions))]
pub(crate) use trace;
