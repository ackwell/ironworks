use std::vec::Vec;
use crate::utility::read_array;
use crate::error::PopulateError;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use std::result::Result;
impl MetadataAdapter for MJIRecipe {
    fn name() -> String {
        "MJIRecipe".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIRecipe::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIRecipe_t {
    pub r#material: u8,
    pub r#amount: u8,
}
impl MJIRecipe_t {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#material: row.field(4usize + offset)?.into_u8()?,
            r#amount: row.field(5usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct MJIRecipe {
    pub r#log_message: u32,
    pub r#key_item: u8,
    pub r#item_pouch: u8,
    pub r#unknown3: u8,
    pub r#t: Vec<MJIRecipe_t>,
    pub r#order: u8,
}
impl MJIRecipe {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#log_message: row.field(0usize + offset)?.into_u32()?,
            r#key_item: row.field(1usize + offset)?.into_u8()?,
            r#item_pouch: row.field(2usize + offset)?.into_u8()?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#t: read_array(
                offset,
                5usize,
                2usize,
                |offset| { Result::Ok(MJIRecipe_t::populate(row, offset)?) },
            )?,
            r#order: row.field(14usize + offset)?.into_u8()?,
        })
    }
}
