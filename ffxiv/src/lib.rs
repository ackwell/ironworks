/*!
Collection of helper for working with FFXIV data utilising the Ironworks toolkit.
*/

#![warn(missing_debug_implementations, missing_docs)]

// TODO: is it honestly worth seperating xiv stuff? really?

mod excel;
mod sqpack;

pub use excel::{ExcelSqPack, Language};
pub use sqpack::SqPackFfxiv;
