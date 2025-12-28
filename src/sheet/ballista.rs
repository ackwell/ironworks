use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Ballista {
    fn name() -> String {
        "Ballista".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Ballista::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Ballista {
    pub r#bnpc: u16,
    pub r#near: i8,
    pub r#far: i8,
    pub r#angle: u16,
    pub r#bullet: u8,
    pub r#unknown5: u8,
    pub r#unknown6: u8,
    pub r#action0: u16,
    pub r#action1: u16,
    pub r#action2: u16,
    pub r#action3: u16,
}
impl Ballista {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#bnpc: row.field(0usize + offset)?.into_u16()?,
            r#near: row.field(1usize + offset)?.into_i8()?,
            r#far: row.field(2usize + offset)?.into_i8()?,
            r#angle: row.field(3usize + offset)?.into_u16()?,
            r#bullet: row.field(4usize + offset)?.into_u8()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_u8()?,
            r#action0: row.field(7usize + offset)?.into_u16()?,
            r#action1: row.field(8usize + offset)?.into_u16()?,
            r#action2: row.field(9usize + offset)?.into_u16()?,
            r#action3: row.field(10usize + offset)?.into_u16()?,
        })
    }
}
