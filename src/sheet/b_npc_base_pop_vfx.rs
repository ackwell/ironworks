use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for BNpcBasePopVfx {
    fn name() -> String {
        "BNpcBasePopVfx".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(BNpcBasePopVfx::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct BNpcBasePopVfx {
    pub r#unknown0: u16,
}
impl BNpcBasePopVfx {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u16()?,
        })
    }
}
