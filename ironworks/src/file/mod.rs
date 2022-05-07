//! Collection of pre-defined file readers for known file formats.
//!
//! Each file type may contain a number of related supporting items, and as such are namespaced seperately.

mod file;

#[cfg(feature = "exd")]
pub mod exd;
#[cfg(feature = "exh")]
pub mod exh;
#[cfg(feature = "exl")]
pub mod exl;
#[cfg(feature = "tex")]
pub mod tex;

pub use file::File;
