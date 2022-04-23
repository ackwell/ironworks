use crate::metadata::MetadataAdapter;
use crate::error::PopulateError;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for Transformation {
    fn name() -> String {
        "Transformation".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Transformation::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Transformation {
    pub r#unknown0: u8,
    pub r#model: i16,
    pub r#b_npc_name: u16,
    pub r#b_npc_customize: i32,
    pub r#npc_equip: i32,
    pub r#ex_hotbar_enable_config: bool,
    pub r#action0: u16,
    pub r#unknown7: bool,
    pub r#action1: u16,
    pub r#unknown9: bool,
    pub r#action2: u16,
    pub r#unknown11: bool,
    pub r#action3: u16,
    pub r#unknown13: bool,
    pub r#action4: u16,
    pub r#unknown15: bool,
    pub r#action5: u16,
    pub r#unknown17: bool,
    pub r#rp_parameter: u16,
    pub r#remove_action: u16,
    pub r#unknown20: bool,
    pub r#unknown21: bool,
    pub r#unknown22: u8,
    pub r#unknown23: bool,
    pub r#speed: f32,
    pub r#scale: f32,
    pub r#is_pv_p: bool,
    pub r#is_event: bool,
    pub r#player_camera: bool,
    pub r#unknown29: bool,
    pub r#unknown30: bool,
    pub r#start_vfx: u16,
    pub r#end_vfx: u16,
    pub r#action6: u32,
    pub r#unknown34: i8,
    pub r#unknown35: i8,
    pub r#action7: u16,
}
impl Transformation {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_u8()?,
            r#model: row.field(1usize + offset)?.into_i16()?,
            r#b_npc_name: row.field(2usize + offset)?.into_u16()?,
            r#b_npc_customize: row.field(3usize + offset)?.into_i32()?,
            r#npc_equip: row.field(4usize + offset)?.into_i32()?,
            r#ex_hotbar_enable_config: row.field(5usize + offset)?.into_bool()?,
            r#action0: row.field(6usize + offset)?.into_u16()?,
            r#unknown7: row.field(7usize + offset)?.into_bool()?,
            r#action1: row.field(8usize + offset)?.into_u16()?,
            r#unknown9: row.field(9usize + offset)?.into_bool()?,
            r#action2: row.field(10usize + offset)?.into_u16()?,
            r#unknown11: row.field(11usize + offset)?.into_bool()?,
            r#action3: row.field(12usize + offset)?.into_u16()?,
            r#unknown13: row.field(13usize + offset)?.into_bool()?,
            r#action4: row.field(14usize + offset)?.into_u16()?,
            r#unknown15: row.field(15usize + offset)?.into_bool()?,
            r#action5: row.field(16usize + offset)?.into_u16()?,
            r#unknown17: row.field(17usize + offset)?.into_bool()?,
            r#rp_parameter: row.field(18usize + offset)?.into_u16()?,
            r#remove_action: row.field(19usize + offset)?.into_u16()?,
            r#unknown20: row.field(20usize + offset)?.into_bool()?,
            r#unknown21: row.field(21usize + offset)?.into_bool()?,
            r#unknown22: row.field(22usize + offset)?.into_u8()?,
            r#unknown23: row.field(23usize + offset)?.into_bool()?,
            r#speed: row.field(24usize + offset)?.into_f32()?,
            r#scale: row.field(25usize + offset)?.into_f32()?,
            r#is_pv_p: row.field(26usize + offset)?.into_bool()?,
            r#is_event: row.field(27usize + offset)?.into_bool()?,
            r#player_camera: row.field(28usize + offset)?.into_bool()?,
            r#unknown29: row.field(29usize + offset)?.into_bool()?,
            r#unknown30: row.field(30usize + offset)?.into_bool()?,
            r#start_vfx: row.field(31usize + offset)?.into_u16()?,
            r#end_vfx: row.field(32usize + offset)?.into_u16()?,
            r#action6: row.field(33usize + offset)?.into_u32()?,
            r#unknown34: row.field(34usize + offset)?.into_i8()?,
            r#unknown35: row.field(35usize + offset)?.into_i8()?,
            r#action7: row.field(36usize + offset)?.into_u16()?,
        })
    }
}
