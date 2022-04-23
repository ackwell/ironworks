use std::vec::Vec;
use crate::error::PopulateError;
use crate::utility::read_array;
use std::result::Result;
use ironworks::excel::Row;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for PhysicsGroup {
    fn name() -> String {
        "PhysicsGroup".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(PhysicsGroup::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct PhysicsGroup {
    pub r#simulation_time: Vec<f32>,
    pub r#ps3_simulation_time: Vec<f32>,
    pub r#reset_by_look_at: bool,
    pub r#root_following_game: f32,
    pub r#root_following_cut_scene: f32,
    pub r#config_switch: Vec<i8>,
    pub r#force_attract_by_physics_off: bool,
}
impl PhysicsGroup {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#simulation_time: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(0usize + offset)?.into_f32()?) },
            )?,
            r#ps3_simulation_time: read_array(
                offset,
                6usize,
                1usize,
                |offset| { Result::Ok(row.field(6usize + offset)?.into_f32()?) },
            )?,
            r#reset_by_look_at: row.field(12usize + offset)?.into_bool()?,
            r#root_following_game: row.field(13usize + offset)?.into_f32()?,
            r#root_following_cut_scene: row.field(14usize + offset)?.into_f32()?,
            r#config_switch: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(15usize + offset)?.into_i8()?) },
            )?,
            r#force_attract_by_physics_off: row.field(18usize + offset)?.into_bool()?,
        })
    }
}
