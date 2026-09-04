//! Collection of pre-defined file readers for known file formats.
//!
//! Each file type may contain a number of related supporting items, and as such are namespaced seperately.

#[cfg(any(feature = "sklb", feature = "skp"))]
mod animation;
#[cfg(any(feature = "eqp", feature = "gmp"))]
mod block_table;
mod file;
// sklb reads only the skeleton out of a tagfile and pap only the animations.
#[cfg(any(feature = "pap", feature = "sklb"))]
#[cfg_attr(not(all(feature = "pap", feature = "sklb")), allow(dead_code))]
mod havok;
#[cfg(feature = "layer")]
pub mod layer;
#[cfg(any(feature = "shcd", feature = "shpk"))]
mod shader;

#[cfg(feature = "amb")]
pub mod amb;
#[cfg(feature = "atch")]
pub mod atch;
#[cfg(feature = "avfx")]
pub mod avfx;
#[cfg(feature = "cmp")]
pub mod cmp;
#[cfg(feature = "cutb")]
pub mod cutb;
#[cfg(feature = "dic")]
pub mod dic;
#[cfg(feature = "eid")]
pub mod eid;
#[cfg(feature = "envb")]
pub mod envb;
#[cfg(any(feature = "envb", feature = "essb", feature = "obsb"))]
pub mod envs;
#[cfg(feature = "eqdp")]
pub mod eqdp;
#[cfg(feature = "eqp")]
pub mod eqp;
#[cfg(feature = "essb")]
pub mod essb;
#[cfg(feature = "est")]
pub mod est;
#[cfg(feature = "evp")]
pub mod evp;
#[cfg(feature = "exd")]
pub mod exd;
#[cfg(feature = "exh")]
pub mod exh;
#[cfg(feature = "exl")]
pub mod exl;
#[cfg(feature = "fdt")]
pub mod fdt;
#[cfg(feature = "gfd")]
pub mod gfd;
#[cfg(feature = "ggd")]
pub mod ggd;
#[cfg(feature = "gmp")]
pub mod gmp;
#[cfg(feature = "gzd")]
pub mod gzd;
#[cfg(feature = "hwc")]
pub mod hwc;
#[cfg(feature = "imc")]
pub mod imc;
#[cfg(feature = "lcb")]
pub mod lcb;
#[cfg(feature = "lgb")]
pub mod lgb;
#[cfg(feature = "luab")]
pub mod luab;
#[cfg(feature = "lvb")]
pub mod lvb;
#[cfg(feature = "mdl")]
pub mod mdl;
#[cfg(feature = "mtrl")]
pub mod mtrl;
#[cfg(feature = "obsb")]
pub mod obsb;
#[cfg(feature = "pap")]
pub mod pap;
#[cfg(feature = "patch")]
pub mod patch;
#[cfg(feature = "pbd")]
pub mod pbd;
#[cfg(feature = "pcb")]
pub mod pcb;
#[cfg(feature = "phyb")]
pub mod phyb;
#[cfg(feature = "scd")]
pub mod scd;
#[cfg(feature = "sgb")]
pub mod sgb;
#[cfg(feature = "shcd")]
pub mod shcd;
#[cfg(feature = "shpk")]
pub mod shpk;
#[cfg(feature = "sklb")]
pub mod sklb;
#[cfg(feature = "skp")]
pub mod skp;
#[cfg(feature = "spm")]
pub mod spm;
#[cfg(feature = "stm")]
pub mod stm;
#[cfg(feature = "svb")]
pub mod svb;
#[cfg(feature = "tera")]
pub mod tera;
#[cfg(feature = "tex")]
pub mod tex;
#[cfg(feature = "tmb")]
pub mod tmb;
#[cfg(feature = "uld")]
pub mod uld;
#[cfg(feature = "uwb")]
pub mod uwb;

pub use file::File;
