use crate::metadata::MetadataAdapter;
use std::result::Result;
use crate::error::PopulateError;
use ironworks::excel::Row;
impl MetadataAdapter for GimmickJump {
    fn name() -> String {
        "GimmickJump".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(GimmickJump::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct GimmickJump {
    pub r#fall_damage: u16,
    pub r#height: i8,
    pub r#loop_motion: u32,
    pub r#end_motion: u32,
    pub r#start_client: bool,
}
impl GimmickJump {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#fall_damage: row.field(0usize + offset)?.into_u16()?,
            r#height: row.field(1usize + offset)?.into_i8()?,
            r#loop_motion: row.field(2usize + offset)?.into_u32()?,
            r#end_motion: row.field(3usize + offset)?.into_u32()?,
            r#start_client: row.field(4usize + offset)?.into_bool()?,
        })
    }
}
