use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for ItemAction {
    fn name() -> String {
        "ItemAction".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ItemAction::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ItemAction {
    pub r#cond_lv: u8,
    pub r#cond_battle: bool,
    pub r#cond_pvp: bool,
    pub r#cond_pvp_only: bool,
    pub r#type: u16,
    pub r#data: Vec<u16>,
    pub r#data_hq: Vec<u16>,
}
impl ItemAction {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#cond_lv: row.field(0usize + offset)?.into_u8()?,
            r#cond_battle: row.field(1usize + offset)?.into_bool()?,
            r#cond_pvp: row.field(2usize + offset)?.into_bool()?,
            r#cond_pvp_only: row.field(3usize + offset)?.into_bool()?,
            r#type: row.field(4usize + offset)?.into_u16()?,
            r#data: read_array(
                offset,
                9usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u16()?) },
            )?,
            r#data_hq: read_array(
                offset,
                9usize,
                1usize,
                |offset| { Result::Ok(row.field(14usize + offset)?.into_u16()?) },
            )?,
        })
    }
}
