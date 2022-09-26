use std::vec::Vec;
use crate::utility::read_array;
use crate::error::PopulateError;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
impl MetadataAdapter for Recipe {
    fn name() -> String {
        "Recipe".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(Recipe::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct Recipe_mIngredient {
    pub r#item_ingredient: i32,
    pub r#amount_ingredient: u8,
}
impl Recipe_mIngredient {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#item_ingredient: row.field(5usize + offset)?.into_i32()?,
            r#amount_ingredient: row.field(6usize + offset)?.into_u8()?,
        })
    }
}
#[derive(Debug)]
pub struct Recipe {
    pub r#number: i32,
    pub r#craft_type: i32,
    pub r#recipe_level_table: u16,
    pub r#item_result: i32,
    pub r#amount_result: u8,
    pub r#m_ingredient: Vec<Recipe_mIngredient>,
    pub r#recipe_notebook_list: u16,
    pub r#is_secondary: bool,
    pub r#material_quality_factor: u8,
    pub r#difficulty_factor: u16,
    pub r#quality_factor: u16,
    pub r#durability_factor: u16,
    pub r#unknown31: u16,
    pub r#required_quality: u32,
    pub r#required_craftsmanship: u16,
    pub r#required_control: u16,
    pub r#quick_synth_craftsmanship: u16,
    pub r#quick_synth_control: u16,
    pub r#secret_recipe_book: u16,
    pub r#quest: u32,
    pub r#can_quick_synth: bool,
    pub r#can_hq: bool,
    pub r#exp_rewarded: bool,
    pub r#status_required: i32,
    pub r#item_required: i32,
    pub r#is_specialization_required: bool,
    pub r#is_expert: bool,
    pub r#unknown46: u8,
    pub r#patch_number: u16,
}
impl Recipe {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#number: row.field(0usize + offset)?.into_i32()?,
            r#craft_type: row.field(1usize + offset)?.into_i32()?,
            r#recipe_level_table: row.field(2usize + offset)?.into_u16()?,
            r#item_result: row.field(3usize + offset)?.into_i32()?,
            r#amount_result: row.field(4usize + offset)?.into_u8()?,
            r#m_ingredient: read_array(
                offset,
                10usize,
                2usize,
                |offset| { Result::Ok(Recipe_mIngredient::populate(row, offset)?) },
            )?,
            r#recipe_notebook_list: row.field(25usize + offset)?.into_u16()?,
            r#is_secondary: row.field(26usize + offset)?.into_bool()?,
            r#material_quality_factor: row.field(27usize + offset)?.into_u8()?,
            r#difficulty_factor: row.field(28usize + offset)?.into_u16()?,
            r#quality_factor: row.field(29usize + offset)?.into_u16()?,
            r#durability_factor: row.field(30usize + offset)?.into_u16()?,
            r#unknown31: row.field(31usize + offset)?.into_u16()?,
            r#required_quality: row.field(32usize + offset)?.into_u32()?,
            r#required_craftsmanship: row.field(33usize + offset)?.into_u16()?,
            r#required_control: row.field(34usize + offset)?.into_u16()?,
            r#quick_synth_craftsmanship: row.field(35usize + offset)?.into_u16()?,
            r#quick_synth_control: row.field(36usize + offset)?.into_u16()?,
            r#secret_recipe_book: row.field(37usize + offset)?.into_u16()?,
            r#quest: row.field(38usize + offset)?.into_u32()?,
            r#can_quick_synth: row.field(39usize + offset)?.into_bool()?,
            r#can_hq: row.field(40usize + offset)?.into_bool()?,
            r#exp_rewarded: row.field(41usize + offset)?.into_bool()?,
            r#status_required: row.field(42usize + offset)?.into_i32()?,
            r#item_required: row.field(43usize + offset)?.into_i32()?,
            r#is_specialization_required: row.field(44usize + offset)?.into_bool()?,
            r#is_expert: row.field(45usize + offset)?.into_bool()?,
            r#unknown46: row.field(46usize + offset)?.into_u8()?,
            r#patch_number: row.field(47usize + offset)?.into_u16()?,
        })
    }
}
