use std::result::Result;
use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for EmoteMode {
    fn name() -> String {
        "EmoteMode".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(EmoteMode::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct EmoteMode {
    pub r#start_emote: u16,
    pub r#end_emote: u16,
    pub r#move: bool,
    pub r#camera: bool,
    pub r#end_on_rotate: bool,
    pub r#end_on_emote: bool,
    pub r#condition_mode: u8,
}
impl EmoteMode {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#start_emote: row.field(0usize + offset)?.into_u16()?,
            r#end_emote: row.field(1usize + offset)?.into_u16()?,
            r#move: row.field(2usize + offset)?.into_bool()?,
            r#camera: row.field(3usize + offset)?.into_bool()?,
            r#end_on_rotate: row.field(4usize + offset)?.into_bool()?,
            r#end_on_emote: row.field(5usize + offset)?.into_bool()?,
            r#condition_mode: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
