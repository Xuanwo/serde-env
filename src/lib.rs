//! serde-env will allow deserializing env into structs.
//!
//! The biggest different between [envy](https://github.com/softprops/envy) is
//! serde-env supports deserialize `_` seperated env into nests structs. That means
//! we will treat env as `_` seperated tree instead of a flat map.
//!
//! For examples:
//!
//! ```
//! use serde::Deserialize;
//! use serde_env::from_env;
//!
//! #[derive(Debug, Deserialize)]
//! struct Cargo {
//!     home: String,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Test {
//!     #[cfg(windows)]
//!     #[serde(rename="userprofile")]
//!     home: String,
//!     #[cfg(not(windows))]
//!     home: String,
//!     cargo: Cargo,
//! }
//!
//! let t: Test = from_env().expect("deserialize from env");
//!
//! assert!(!t.home.is_empty());
//! assert!(!t.cargo.home.is_empty());
//! println!("{:?}", t)
//! ```

mod cond_log;
mod de;
mod error;
mod value;

pub use de::from_env;
