use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for FCActivity {
    fn name() -> String {
        "FCActivity".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(FCActivity::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct FCActivity {
    pub r#text: SeString,
    pub r#self_kind: u8,
    pub r#target_kind: u8,
    pub r#num_param: u8,
    pub r#fc_activity_category: u8,
    pub r#icon_type: i8,
}
impl FCActivity {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#text: row.field(0usize + offset)?.into_string()?,
            r#self_kind: row.field(1usize + offset)?.into_u8()?,
            r#target_kind: row.field(2usize + offset)?.into_u8()?,
            r#num_param: row.field(3usize + offset)?.into_u8()?,
            r#fc_activity_category: row.field(4usize + offset)?.into_u8()?,
            r#icon_type: row.field(5usize + offset)?.into_i8()?,
        })
    }
}
