#[cfg(feature = "client")]
pub mod client;
pub mod iomod;
pub mod event;

#[cfg(feature = "host")]
extern crate wasmer_runtime;

/* Cloud interface */

pub trait Database {
    // TODO: general-purpose database api ?
}

