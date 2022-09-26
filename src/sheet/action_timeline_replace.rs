use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for ActionTimelineReplace {
    fn name() -> String {
        "ActionTimelineReplace".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ActionTimelineReplace::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ActionTimelineReplace {
    pub r#old: u16,
    pub r#new: u16,
}
impl ActionTimelineReplace {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#old: row.field(0usize + offset)?.into_u16()?,
            r#new: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
