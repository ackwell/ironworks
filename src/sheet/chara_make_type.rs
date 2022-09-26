use crate::utility::read_array;
use crate::error::PopulateError;
use std::result::Result;
use ironworks::excel::Row;
use std::vec::Vec;
use crate::metadata::MetadataAdapter;
impl MetadataAdapter for CharaMakeType {
    fn name() -> String {
        "CharaMakeType".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaMakeType::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaMakeType_SubMenuParam {
    pub r#sub_menu_param: Vec<u32>,
}
impl CharaMakeType_SubMenuParam {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sub_menu_param: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(199usize + offset)?.into_u32()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct CharaMakeType_SubMenuGraphic {
    pub r#sub_menu_graphic: Vec<u8>,
}
impl CharaMakeType_SubMenuGraphic {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#sub_menu_graphic: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(2999usize + offset)?.into_u8()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct CharaMakeType_Unnamed3347 {
    pub r#helmet: Vec<u64>,
    pub r#top: Vec<u64>,
    pub r#gloves: Vec<u64>,
    pub r#legs: Vec<u64>,
    pub r#shoes: Vec<u64>,
    pub r#weapon: Vec<u64>,
    pub r#sub_weapon: Vec<u64>,
}
impl CharaMakeType_Unnamed3347 {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#helmet: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3347usize + offset)?.into_u64()?) },
            )?,
            r#top: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3350usize + offset)?.into_u64()?) },
            )?,
            r#gloves: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3353usize + offset)?.into_u64()?) },
            )?,
            r#legs: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3356usize + offset)?.into_u64()?) },
            )?,
            r#shoes: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3359usize + offset)?.into_u64()?) },
            )?,
            r#weapon: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3362usize + offset)?.into_u64()?) },
            )?,
            r#sub_weapon: read_array(
                offset,
                3usize,
                1usize,
                |offset| { Result::Ok(row.field(3365usize + offset)?.into_u64()?) },
            )?,
        })
    }
}
#[derive(Debug)]
pub struct CharaMakeType {
    pub r#race: i32,
    pub r#tribe: i32,
    pub r#gender: i8,
    pub r#menu: Vec<u32>,
    pub r#init_val: Vec<u8>,
    pub r#sub_menu_type: Vec<u8>,
    pub r#sub_menu_num: Vec<u8>,
    pub r#look_at: Vec<u8>,
    pub r#sub_menu_mask: Vec<u32>,
    pub r#customize: Vec<u32>,
    pub r#sub_menu_param: Vec<CharaMakeType_SubMenuParam>,
    pub r#sub_menu_graphic: Vec<CharaMakeType_SubMenuGraphic>,
    pub r#voice_struct: Vec<u8>,
    pub r#facial_feature_option: Vec<Vec<i32>>,
    pub r#unnamed3347: Vec<CharaMakeType_Unnamed3347>,
}
impl CharaMakeType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#race: row.field(0usize + offset)?.into_i32()?,
            r#tribe: row.field(1usize + offset)?.into_i32()?,
            r#gender: row.field(2usize + offset)?.into_i8()?,
            r#menu: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(3usize + offset)?.into_u32()?) },
            )?,
            r#init_val: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(31usize + offset)?.into_u8()?) },
            )?,
            r#sub_menu_type: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(59usize + offset)?.into_u8()?) },
            )?,
            r#sub_menu_num: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(87usize + offset)?.into_u8()?) },
            )?,
            r#look_at: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(115usize + offset)?.into_u8()?) },
            )?,
            r#sub_menu_mask: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(143usize + offset)?.into_u32()?) },
            )?,
            r#customize: read_array(
                offset,
                28usize,
                1usize,
                |offset| { Result::Ok(row.field(171usize + offset)?.into_u32()?) },
            )?,
            r#sub_menu_param: read_array(
                offset,
                100usize,
                28usize,
                |offset| {
                    Result::Ok(CharaMakeType_SubMenuParam::populate(row, offset)?)
                },
            )?,
            r#sub_menu_graphic: read_array(
                offset,
                10usize,
                28usize,
                |offset| {
                    Result::Ok(CharaMakeType_SubMenuGraphic::populate(row, offset)?)
                },
            )?,
            r#voice_struct: read_array(
                offset,
                12usize,
                1usize,
                |offset| { Result::Ok(row.field(3279usize + offset)?.into_u8()?) },
            )?,
            r#facial_feature_option: read_array(
                offset,
                7usize,
                8usize,
                |offset| {
                    Result::Ok(
                        read_array(
                            offset,
                            8usize,
                            1usize,
                            |offset| {
                                Result::Ok(row.field(3291usize + offset)?.into_i32()?)
                            },
                        )?,
                    )
                },
            )?,
            r#unnamed3347: read_array(
                offset,
                1usize,
                21usize,
                |offset| {
                    Result::Ok(CharaMakeType_Unnamed3347::populate(row, offset)?)
                },
            )?,
        })
    }
}
