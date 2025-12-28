use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use std::result::Result;
impl MetadataAdapter for MountCustomize {
    fn name() -> String {
        "MountCustomize".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(MountCustomize::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct MountCustomize {
    pub r#unknown0: bool,
    pub r#hyur_midlander_male_scale: u16,
    pub r#hyur_midlander_female_scale: u16,
    pub r#hyur_highlander_male_scale: u16,
    pub r#hyur_highlander_female_scale: u16,
    pub r#elezen_male_scale: u16,
    pub r#elezen_female_scale: u16,
    pub r#lala_male_scale: u16,
    pub r#lala_female_scale: u16,
    pub r#miqo_male_scale: u16,
    pub r#miqo_female_scale: u16,
    pub r#roe_male_scale: u16,
    pub r#roe_female_scale: u16,
    pub r#au_ra_male_scale: u16,
    pub r#au_ra_female_scale: u16,
    pub r#hrothgar_male_scale: u16,
    pub r#hrothgar_female_scale: u16,
    pub r#viera_male_scale: u16,
    pub r#viera_female_scale: u16,
    pub r#hyur_midlander_male_camera_height: u16,
    pub r#hyur_midlander_female_camera_height: u16,
    pub r#hyur_highlander_male_camera_height: u16,
    pub r#hyur_highlander_female_camera_height: u8,
    pub r#elezen_male_camera_height: u8,
    pub r#elezen_female_camera_height: u8,
    pub r#lala_male_camera_height: u8,
    pub r#lala_female_camera_height: u8,
    pub r#miqo_male_camera_height: u8,
    pub r#miqo_female_camera_height: u8,
    pub r#roe_male_camera_height: u8,
    pub r#roe_female_camera_height: u8,
    pub r#au_ra_male_camera_height: u8,
    pub r#au_ra_female_camera_height: u8,
    pub r#hrothgar_male_camera_height: u8,
    pub r#viera_male_camera_height: u8,
    pub r#viera_female_camera_height: u8,
    pub r#unknown36: u8,
    pub r#unknown37: u8,
    pub r#unknown38: u8,
    pub r#unknown39: u8,
    pub r#unknown40: u8,
    pub r#unknown41: u8,
    pub r#unknown42: u8,
}
impl MountCustomize {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#unknown0: row.field(0usize + offset)?.into_bool()?,
            r#hyur_midlander_male_scale: row.field(1usize + offset)?.into_u16()?,
            r#hyur_midlander_female_scale: row.field(2usize + offset)?.into_u16()?,
            r#hyur_highlander_male_scale: row.field(3usize + offset)?.into_u16()?,
            r#hyur_highlander_female_scale: row.field(4usize + offset)?.into_u16()?,
            r#elezen_male_scale: row.field(5usize + offset)?.into_u16()?,
            r#elezen_female_scale: row.field(6usize + offset)?.into_u16()?,
            r#lala_male_scale: row.field(7usize + offset)?.into_u16()?,
            r#lala_female_scale: row.field(8usize + offset)?.into_u16()?,
            r#miqo_male_scale: row.field(9usize + offset)?.into_u16()?,
            r#miqo_female_scale: row.field(10usize + offset)?.into_u16()?,
            r#roe_male_scale: row.field(11usize + offset)?.into_u16()?,
            r#roe_female_scale: row.field(12usize + offset)?.into_u16()?,
            r#au_ra_male_scale: row.field(13usize + offset)?.into_u16()?,
            r#au_ra_female_scale: row.field(14usize + offset)?.into_u16()?,
            r#hrothgar_male_scale: row.field(15usize + offset)?.into_u16()?,
            r#hrothgar_female_scale: row.field(16usize + offset)?.into_u16()?,
            r#viera_male_scale: row.field(17usize + offset)?.into_u16()?,
            r#viera_female_scale: row.field(18usize + offset)?.into_u16()?,
            r#hyur_midlander_male_camera_height: row
                .field(19usize + offset)?
                .into_u16()?,
            r#hyur_midlander_female_camera_height: row
                .field(20usize + offset)?
                .into_u16()?,
            r#hyur_highlander_male_camera_height: row
                .field(21usize + offset)?
                .into_u16()?,
            r#hyur_highlander_female_camera_height: row
                .field(22usize + offset)?
                .into_u8()?,
            r#elezen_male_camera_height: row.field(23usize + offset)?.into_u8()?,
            r#elezen_female_camera_height: row.field(24usize + offset)?.into_u8()?,
            r#lala_male_camera_height: row.field(25usize + offset)?.into_u8()?,
            r#lala_female_camera_height: row.field(26usize + offset)?.into_u8()?,
            r#miqo_male_camera_height: row.field(27usize + offset)?.into_u8()?,
            r#miqo_female_camera_height: row.field(28usize + offset)?.into_u8()?,
            r#roe_male_camera_height: row.field(29usize + offset)?.into_u8()?,
            r#roe_female_camera_height: row.field(30usize + offset)?.into_u8()?,
            r#au_ra_male_camera_height: row.field(31usize + offset)?.into_u8()?,
            r#au_ra_female_camera_height: row.field(32usize + offset)?.into_u8()?,
            r#hrothgar_male_camera_height: row.field(33usize + offset)?.into_u8()?,
            r#viera_male_camera_height: row.field(34usize + offset)?.into_u8()?,
            r#viera_female_camera_height: row.field(35usize + offset)?.into_u8()?,
            r#unknown36: row.field(36usize + offset)?.into_u8()?,
            r#unknown37: row.field(37usize + offset)?.into_u8()?,
            r#unknown38: row.field(38usize + offset)?.into_u8()?,
            r#unknown39: row.field(39usize + offset)?.into_u8()?,
            r#unknown40: row.field(40usize + offset)?.into_u8()?,
            r#unknown41: row.field(41usize + offset)?.into_u8()?,
            r#unknown42: row.field(42usize + offset)?.into_u8()?,
        })
    }
}
