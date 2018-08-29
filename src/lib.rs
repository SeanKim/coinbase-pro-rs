//! Coinbase pro client with sync and async support
//!
//! ## Structure
//!
//! There are two main structures to work with: [`Private`] and [`Public`], which provide interfaces to
//! work with [https://docs.pro.coinbase.com](https://docs.pro.coinbase.com) .
//! The structures should be parametrised with: [`Sync`] or [`ASync`] adapter-type, which blocks
//! future and returns result of its execution for Sync adapter or returns Future for ASync
//! adapter.
//!
//! ## Example
//!
//! ### Async
//! ```
//! extern crate hyper;
//! extern crate tokio;
//! extern crate coinbase_pro_rs;
//!
//! use hyper::rt::Future;
//! use coinbase_pro_rs::{Public, ASync, SANDBOX_URL};
//!
//! fn main() {
//!     let client: Public<ASync> = Public::new_with_keep_alive(SANDBOX_URL, false);
//!     // if keep_alive is not disables - tokio::run will hold the connection without exiting the example
//!     let f = client.get_time()
//!         .map_err(|_| ())
//!         .and_then(|time| {
//!             println!("Coinbase.time: {}", time.iso);
//!             Ok(())
//!         });
//!
//!     tokio::run(f);
//! }
//! ```
//! ### Sync
//! ```
//! extern crate coinbase_pro_rs;
//!
//! use coinbase_pro_rs::{Public, Sync, SANDBOX_URL};
//!
//! fn main() {
//!    let client: Public<Sync> = Public::new(SANDBOX_URL);
//!    let time = client.get_time().unwrap();
//!    println!("Coinbase.time: {}", time.iso);
//!}
//! ```

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;
extern crate serde;
extern crate time;
extern crate tokio;
extern crate uuid;
extern crate tokio_tungstenite;

pub mod error;
pub mod private;
pub mod public;
pub mod structs;
pub mod adapters;
mod utils;

pub mod wsfeed;

pub use private::Private;
pub use public::Public;
pub use error::CBError;
pub use adapters::{Sync, ASync};

pub type Result<T> = std::result::Result<T, CBError>;

/// https://api.pro.coinbase.com
pub const MAIN_URL: &str = "https://api.pro.coinbase.com";
/// https://api-public.sandbox.pro.coinbase.com
pub const SANDBOX_URL: &str = "https://api-public.sandbox.pro.coinbase.com";


