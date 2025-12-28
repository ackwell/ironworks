use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
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
    pub r#height: u16,
    pub r#loop_motion: u32,
    pub r#end_motion: u32,
    pub r#start_client: bool,
    pub r#unknown5: bool,
    pub r#unknown6: u16,
    pub r#unknown7: u16,
    pub r#unknown8: bool,
}
impl GimmickJump {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#fall_damage: row.field(0usize + offset)?.into_u16()?,
            r#height: row.field(1usize + offset)?.into_u16()?,
            r#loop_motion: row.field(2usize + offset)?.into_u32()?,
            r#end_motion: row.field(3usize + offset)?.into_u32()?,
            r#start_client: row.field(4usize + offset)?.into_bool()?,
            r#unknown5: row.field(5usize + offset)?.into_bool()?,
            r#unknown6: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_bool()?,
        })
    }
}
