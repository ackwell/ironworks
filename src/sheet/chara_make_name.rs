use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
use std::result::Result;
impl MetadataAdapter for CharaMakeName {
    fn name() -> String {
        "CharaMakeName".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CharaMakeName::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CharaMakeName {
    pub r#hyur_midlander_male: SeString,
    pub r#hyur_midlander_female: SeString,
    pub r#hyur_midlander_last_name: SeString,
    pub r#hyur_highlander_male: SeString,
    pub r#hyur_highlander_female: SeString,
    pub r#hyur_highlander_last_name: SeString,
    pub r#elezen_male: SeString,
    pub r#elezen_female: SeString,
    pub r#elezen_wildwood_last_name: SeString,
    pub r#elezen_duskwight_last_name: SeString,
    pub r#miqote_sun_male: SeString,
    pub r#miqote_sun_female: SeString,
    pub r#miqote_sun_male_last_name: SeString,
    pub r#miqote_sun_female_last_name: SeString,
    pub r#miqote_moon_male: SeString,
    pub r#miqote_moon_female: SeString,
    pub r#miqote_moon_lastname: SeString,
    pub r#lalafell_plainsfolk_first_name_start: SeString,
    pub r#lalafell_plainsfolk_last_name_start: SeString,
    pub r#lalafell_plainsfolk_end_of_names: SeString,
    pub r#lalafell_dunesfolk_male: SeString,
    pub r#lalafell_dunesfolk_male_last_name: SeString,
    pub r#lalafell_dunesfolk_female: SeString,
    pub r#lalafell_dunesfolk_female_last_name: SeString,
    pub r#roegadyn_sea_wolf_male: SeString,
    pub r#roegadyn_sea_wolf_male_last_name: SeString,
    pub r#roegadyn_sea_wolf_female: SeString,
    pub r#roegadyn_sea_wolf_female_last_name: SeString,
    pub r#roegadyn_hellsguard_first_name: SeString,
    pub r#roegadyn_hellsguard_male_last_name: SeString,
    pub r#roegadyn_hellsguard_female_last_name: SeString,
    pub r#au_ra_raen_male: SeString,
    pub r#au_ra_raen_female: SeString,
    pub r#au_ra_raen_last_name: SeString,
    pub r#au_ra_xaela_male: SeString,
    pub r#au_ra_xaela_female: SeString,
    pub r#au_ra_xaela_last_name: SeString,
    pub r#hrothgar_hellions_first_name: SeString,
    pub r#hrothgar_hellions_last_name: SeString,
    pub r#hrothgar_lost_first_name: SeString,
    pub r#hrothgar_lost_last_name: SeString,
    pub r#unknown41: SeString,
    pub r#unknown42: SeString,
    pub r#unknown43: SeString,
    pub r#unknown44: SeString,
    pub r#unknown45: SeString,
    pub r#unknown46: SeString,
    pub r#viera_first_name: SeString,
    pub r#viera_rava_last_name: SeString,
    pub r#viera_veena_last_name: SeString,
}
impl CharaMakeName {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#hyur_midlander_male: row.field(0usize + offset)?.into_string()?,
            r#hyur_midlander_female: row.field(1usize + offset)?.into_string()?,
            r#hyur_midlander_last_name: row.field(2usize + offset)?.into_string()?,
            r#hyur_highlander_male: row.field(3usize + offset)?.into_string()?,
            r#hyur_highlander_female: row.field(4usize + offset)?.into_string()?,
            r#hyur_highlander_last_name: row.field(5usize + offset)?.into_string()?,
            r#elezen_male: row.field(6usize + offset)?.into_string()?,
            r#elezen_female: row.field(7usize + offset)?.into_string()?,
            r#elezen_wildwood_last_name: row.field(8usize + offset)?.into_string()?,
            r#elezen_duskwight_last_name: row.field(9usize + offset)?.into_string()?,
            r#miqote_sun_male: row.field(10usize + offset)?.into_string()?,
            r#miqote_sun_female: row.field(11usize + offset)?.into_string()?,
            r#miqote_sun_male_last_name: row.field(12usize + offset)?.into_string()?,
            r#miqote_sun_female_last_name: row.field(13usize + offset)?.into_string()?,
            r#miqote_moon_male: row.field(14usize + offset)?.into_string()?,
            r#miqote_moon_female: row.field(15usize + offset)?.into_string()?,
            r#miqote_moon_lastname: row.field(16usize + offset)?.into_string()?,
            r#lalafell_plainsfolk_first_name_start: row
                .field(17usize + offset)?
                .into_string()?,
            r#lalafell_plainsfolk_last_name_start: row
                .field(18usize + offset)?
                .into_string()?,
            r#lalafell_plainsfolk_end_of_names: row
                .field(19usize + offset)?
                .into_string()?,
            r#lalafell_dunesfolk_male: row.field(20usize + offset)?.into_string()?,
            r#lalafell_dunesfolk_male_last_name: row
                .field(21usize + offset)?
                .into_string()?,
            r#lalafell_dunesfolk_female: row.field(22usize + offset)?.into_string()?,
            r#lalafell_dunesfolk_female_last_name: row
                .field(23usize + offset)?
                .into_string()?,
            r#roegadyn_sea_wolf_male: row.field(24usize + offset)?.into_string()?,
            r#roegadyn_sea_wolf_male_last_name: row
                .field(25usize + offset)?
                .into_string()?,
            r#roegadyn_sea_wolf_female: row.field(26usize + offset)?.into_string()?,
            r#roegadyn_sea_wolf_female_last_name: row
                .field(27usize + offset)?
                .into_string()?,
            r#roegadyn_hellsguard_first_name: row
                .field(28usize + offset)?
                .into_string()?,
            r#roegadyn_hellsguard_male_last_name: row
                .field(29usize + offset)?
                .into_string()?,
            r#roegadyn_hellsguard_female_last_name: row
                .field(30usize + offset)?
                .into_string()?,
            r#au_ra_raen_male: row.field(31usize + offset)?.into_string()?,
            r#au_ra_raen_female: row.field(32usize + offset)?.into_string()?,
            r#au_ra_raen_last_name: row.field(33usize + offset)?.into_string()?,
            r#au_ra_xaela_male: row.field(34usize + offset)?.into_string()?,
            r#au_ra_xaela_female: row.field(35usize + offset)?.into_string()?,
            r#au_ra_xaela_last_name: row.field(36usize + offset)?.into_string()?,
            r#hrothgar_hellions_first_name: row.field(37usize + offset)?.into_string()?,
            r#hrothgar_hellions_last_name: row.field(38usize + offset)?.into_string()?,
            r#hrothgar_lost_first_name: row.field(39usize + offset)?.into_string()?,
            r#hrothgar_lost_last_name: row.field(40usize + offset)?.into_string()?,
            r#unknown41: row.field(41usize + offset)?.into_string()?,
            r#unknown42: row.field(42usize + offset)?.into_string()?,
            r#unknown43: row.field(43usize + offset)?.into_string()?,
            r#unknown44: row.field(44usize + offset)?.into_string()?,
            r#unknown45: row.field(45usize + offset)?.into_string()?,
            r#unknown46: row.field(46usize + offset)?.into_string()?,
            r#viera_first_name: row.field(47usize + offset)?.into_string()?,
            r#viera_rava_last_name: row.field(48usize + offset)?.into_string()?,
            r#viera_veena_last_name: row.field(49usize + offset)?.into_string()?,
        })
    }
}
