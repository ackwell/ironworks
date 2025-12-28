use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BNpcAnnounceIcon {
    fn name() -> String {
        "BNpcAnnounceIcon".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BNpcAnnounceIcon::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BNpcAnnounceIcon {
    pub r#icon: u32,
}
impl BNpcAnnounceIcon {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#icon: row.field(0usize + offset)?.into_u32()?,
        })
    }
}
