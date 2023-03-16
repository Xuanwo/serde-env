#[cfg(feature = "log")]
pub(crate) use log::debug;

#[cfg(feature = "log")]
pub(crate) use log::trace;

#[cfg(not(feature = "log"))]
macro_rules! debug {
    ($($arg:tt)+) => {
        // Debug logging disabled in `release` profile
        // Effectively a noop - result ignored by rustc in `release` profile
        format_args!($($arg)+)
    };
}

#[cfg(not(feature = "log"))]
pub(crate) use debug;

#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($($arg:tt)+) => {
        // Trace logging disabled in `release` profile
        // Effectively a noop - result ignored by rustc in `release` profile
        format_args!($($arg)+)
    };
}

#[cfg(not(feature = "log"))]
pub(crate) use trace;
