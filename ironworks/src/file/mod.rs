//! Collection of pre-defined file readers for known file formats.
//!
//! Each file type may contain a number of related supporting items, and as such are namespaced seperately.

mod file;

#[cfg(feature = "eqdp")]
pub mod eqdp;
#[cfg(feature = "exd")]
pub mod exd;
#[cfg(feature = "exh")]
pub mod exh;
#[cfg(feature = "exl")]
pub mod exl;
#[cfg(feature = "mdl")]
pub mod mdl;
#[cfg(feature = "mtrl")]
pub mod mtrl;
#[cfg(feature = "pbd")]
pub mod pbd;
#[cfg(feature = "sklb")]
pub mod sklb;
#[cfg(feature = "tex")]
pub mod tex;
#[cfg(feature = "uld")]
pub mod uld;

pub use file::File;
