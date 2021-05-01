#[macro_use]
extern crate paste;
#[macro_use]
pub mod endpoint_macros;

#[cfg(feature = "with-isahc")]
pub use isahc;

pub mod client;
pub mod endpoint;
#[cfg(feature = "with-isahc")]
pub mod isahc_client;
pub mod v1;

pub use client::Client;
#[cfg(feature = "with-isahc")]
pub use isahc_client::IsahcClient;
