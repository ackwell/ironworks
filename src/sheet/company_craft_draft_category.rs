use std::vec::Vec;
use std::result::Result;
use crate::metadata::MetadataAdapter;
use ironworks::sestring::SeString;
use crate::error::PopulateError;
use crate::utility::read_array;
use ironworks::excel::Row;
impl MetadataAdapter for CompanyCraftDraftCategory {
    fn name() -> String {
        "CompanyCraftDraftCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(CompanyCraftDraftCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct CompanyCraftDraftCategory_CompanyCraftType {
    pub r#company_craft_type: u16,
}
impl CompanyCraftDraftCategory_CompanyCraftType {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#company_craft_type: row.field(1usize + offset)?.into_u16()?,
        })
    }
}
#[derive(Debug)]
pub struct CompanyCraftDraftCategory {
    pub r#name: SeString,
    pub r#company_craft_type: Vec<CompanyCraftDraftCategory_CompanyCraftType>,
}
impl CompanyCraftDraftCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#company_craft_type: read_array(
                offset,
                10usize,
                1usize,
                |offset| {
                    Result::Ok(
                        CompanyCraftDraftCategory_CompanyCraftType::populate(
                            row,
                            offset,
                        )?,
                    )
                },
            )?,
        })
    }
}
