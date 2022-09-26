use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::convert::Infallible;
use std::result::Result;
impl MetadataAdapter for ModelSkeleton {
    fn name() -> String {
        "ModelSkeleton".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ModelSkeleton::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ModelSkeleton {
    pub r#radius: f32,
    pub r#height: f32,
    pub r#vfx_scale: f32,
    pub r#unknown3: u16,
    pub r#unknown4: u16,
    pub r#unknown5: u16,
    pub r#unknown6: u16,
    pub r#unknown7: u16,
    pub r#unknown8: u16,
    pub r#unknown9: u16,
    pub r#unknown10: u16,
    pub r#float_height: f32,
    pub r#float_down: f32,
    pub r#float_up: u16,
    pub r#unknown14: u8,
    pub r#motion_blend_type: bool,
    pub r#loop_fly_se: u8,
    pub r#auto_attack_type: Option<Infallible>,
}
impl ModelSkeleton {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#radius: row.field(0usize + offset)?.into_f32()?,
            r#height: row.field(1usize + offset)?.into_f32()?,
            r#vfx_scale: row.field(2usize + offset)?.into_f32()?,
            r#unknown3: row.field(3usize + offset)?.into_u16()?,
            r#unknown4: row.field(4usize + offset)?.into_u16()?,
            r#unknown5: row.field(5usize + offset)?.into_u16()?,
            r#unknown6: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_u16()?,
            r#unknown8: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_u16()?,
            r#unknown10: row.field(10usize + offset)?.into_u16()?,
            r#float_height: row.field(11usize + offset)?.into_f32()?,
            r#float_down: row.field(12usize + offset)?.into_f32()?,
            r#float_up: row.field(13usize + offset)?.into_u16()?,
            r#unknown14: row.field(14usize + offset)?.into_u8()?,
            r#motion_blend_type: row.field(15usize + offset)?.into_bool()?,
            r#loop_fly_se: row.field(16usize + offset)?.into_u8()?,
            r#auto_attack_type: None,
        })
    }
}
