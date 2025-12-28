use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MJIVillageAppearanceSG {
    fn name() -> String {
        "MJIVillageAppearanceSG".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJIVillageAppearanceSG::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJIVillageAppearanceSG {
    pub r#sgb: Vec<u16>,
    pub r#unknown5: u32,
    pub r#unknown6: u32,
    pub r#unknown7: u32,
    pub r#unknown8: u32,
    pub r#unknown9: u32,
}
impl MJIVillageAppearanceSG {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sgb: read_array(
                offset,
                5usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_u16()?) },
            )?,
            r#unknown5: row.field(5usize + offset)?.into_u32()?,
            r#unknown6: row.field(6usize + offset)?.into_u32()?,
            r#unknown7: row.field(7usize + offset)?.into_u32()?,
            r#unknown8: row.field(8usize + offset)?.into_u32()?,
            r#unknown9: row.field(9usize + offset)?.into_u32()?,
        })
    }
}
