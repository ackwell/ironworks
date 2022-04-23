use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
use crate::error::PopulateError;
impl MetadataAdapter for ModelChara {
    fn name() -> String {
        "ModelChara".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ModelChara::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ModelChara {
    pub r#type: u8,
    pub r#model: u16,
    pub r#base: u8,
    pub r#variant: u8,
    pub r#se_pack: u16,
    pub r#unknown5: u8,
    pub r#unknown6: bool,
    pub r#pap_variation: bool,
}
impl ModelChara {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#type: row.field(0usize + offset)?.into_u8()?,
            r#model: row.field(1usize + offset)?.into_u16()?,
            r#base: row.field(2usize + offset)?.into_u8()?,
            r#variant: row.field(3usize + offset)?.into_u8()?,
            r#se_pack: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u8()?,
            r#unknown6: row.field(6usize + offset)?.into_bool()?,
            r#pap_variation: row.field(7usize + offset)?.into_bool()?,
        })
    }
}
