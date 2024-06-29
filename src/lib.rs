//! serde-env will allow deserializing env into structs.
//!
//! The biggest different between [envy](https://github.com/softprops/envy) is
//! serde-env supports deserialize `_` separated env into nests structs. That means
//! we will treat env as `_` separated tree instead of a flat map.
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
//!     #[serde(rename = "userprofile")]
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

mod de;
pub mod error;
mod value;

pub use de::{from_env, from_env_with_prefix, from_iter, from_iter_with_prefix};
pub use error::Error;
