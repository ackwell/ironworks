use std::result::Result;
use ironworks::sestring::SeString;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::vec::Vec;
impl MetadataAdapter for IKDRoute {
    fn name() -> String {
        "IKDRoute".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(IKDRoute::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct IKDRoute_Unnamed0 {
    pub r#spot: u32,
    pub r#time: u8,
}
impl IKDRoute_Unnamed0 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#spot: row.field(0usize + offset)?.into_u32()?,
            r#time: row.field(1usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct IKDRoute {
    pub r#unnamed0: Vec<IKDRoute_Unnamed0>,
    pub r#image: u32,
    pub r#territory_type: u32,
    pub r#name: SeString,
}
impl IKDRoute {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unnamed0: read_array(
                offset,
                3usize,
                2usize,
                |offset| { Result::Ok(IKDRoute_Unnamed0::populate(row, offset)?) },
            )?,
            r#image: row.field(6usize + offset)?.into_u32()?,
            r#territory_type: row.field(7usize + offset)?.into_u32()?,
            r#name: row.field(8usize + offset)?.into_string()?,
        })
    }
}
