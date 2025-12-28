use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use crate::utility::read_array;
use ironworks::excel::Row;
use std::result::Result;
use std::vec::Vec;
impl MetadataAdapter for MJICraftworksObject {
    fn name() -> String {
        "MJICraftworksObject".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MJICraftworksObject::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MJICraftworksObject_t {
    pub r#material: u16,
    pub r#amount: u16,
}
impl MJICraftworksObject_t {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#material: row.field(4usize + offset)?.into_u16()?,
            r#amount: row.field(5usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct MJICraftworksObject {
    pub r#item: u16,
    pub r#theme: Vec<u16>,
    pub r#unknown3: u16,
    pub r#t: Vec<MJICraftworksObject_t>,
    pub r#level_req: u16,
    pub r#crafting_time: u16,
    pub r#value: u16,
}
impl MJICraftworksObject {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item: row.field(0usize + offset)?.into_u16()?,
            r#theme: read_array(
                offset,
                2usize,
                1usize,
                |offset| { Result::Ok(row.field(1usize + offset)?.into_u16()?) },
            )?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#t: read_array(
                offset,
                4usize,
                2usize,
                |offset| { Result::Ok(MJICraftworksObject_t::populate(row, offset)?) },
            )?,
            r#level_req: row.field(12usize + offset)?.into_u16()?,
            r#crafting_time: row.field(13usize + offset)?.into_u16()?,
            r#value: row.field(14usize + offset)?.into_u16()?,
        })
    }
}
