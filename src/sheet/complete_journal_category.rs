use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
impl MetadataAdapter for CompleteJournalCategory {
    fn name() -> String {
        "CompleteJournalCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompleteJournalCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompleteJournalCategory {
    pub r#first_quest: u32,
    pub r#last_quest: u32,
}
impl CompleteJournalCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#first_quest: row.field(0usize + offset)?.into_u32()?,
            r#last_quest: row.field(1usize + offset)?.into_u32()?,
        })
    }
}
