use crate::utility::read_array;
use crate::error::PopulateError;
use std::vec::Vec;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for ManeuversArmor {
    fn name() -> String {
        "ManeuversArmor".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ManeuversArmor::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ManeuversArmor {
    pub r#unknown0: u16,
    pub r#b_npc_base: Vec<u32>,
    pub r#unknown3: u8,
    pub r#unknown4: bool,
    pub r#icon: Vec<u32>,
}
impl ManeuversArmor {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
            r#b_npc_base: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u32()?) },
            )?,
            r#unknown3: row.field(3usize + offset)?.into_u8()?,
            r#unknown4: row.field(4usize + offset)?.into_bool()?,
            r#icon: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(5usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
