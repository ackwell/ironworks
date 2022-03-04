#![allow(clippy::needless_return)]

mod crc;
mod dat_reader;
mod error;
mod file_struct;
mod index;
mod sqpack;
mod utility;

pub use error::{Error, Result};
pub use sqpack::{Category, Repository, SqPack};
