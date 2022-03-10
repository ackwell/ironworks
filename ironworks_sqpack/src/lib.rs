/*!
General purpose tool for extracting files from SqPack archives.

This crate provides the [SqPack](crate::SqPack) struct, which encapsulates the
indexing and querying of archives within a SqPack database, and the retrieval
of raw files from them.
*/

#![warn(missing_debug_implementations, missing_docs)]

mod crc;
mod dat_reader;
mod error;
mod file_struct;
mod index;
mod sqpack;
mod utility;

pub use error::{Error, Result};
pub use sqpack::{Category, Repository, SqPack};
