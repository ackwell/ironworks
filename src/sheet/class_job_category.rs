use std::result::Result;
use crate::error::PopulateError;
use crate::metadata::MetadataAdapter;
use ironworks::excel::Row;
use ironworks::sestring::SeString;
impl MetadataAdapter for ClassJobCategory {
    fn name() -> String {
        "ClassJobCategory".to_string()
    }
    fn populate(row: &Row) -> Result<Self, PopulateError> {
        let offset = 0;
        Result::Ok(ClassJobCategory::populate(row, offset)?)
    }
}
#[derive(Debug)]
pub struct ClassJobCategory {
    pub r#name: SeString,
    pub r#adv: bool,
    pub r#gla: bool,
    pub r#pgl: bool,
    pub r#mrd: bool,
    pub r#lnc: bool,
    pub r#arc: bool,
    pub r#cnj: bool,
    pub r#thm: bool,
    pub r#crp: bool,
    pub r#bsm: bool,
    pub r#arm: bool,
    pub r#gsm: bool,
    pub r#ltw: bool,
    pub r#wvr: bool,
    pub r#alc: bool,
    pub r#cul: bool,
    pub r#min: bool,
    pub r#btn: bool,
    pub r#fsh: bool,
    pub r#pld: bool,
    pub r#mnk: bool,
    pub r#war: bool,
    pub r#drg: bool,
    pub r#brd: bool,
    pub r#whm: bool,
    pub r#blm: bool,
    pub r#acn: bool,
    pub r#smn: bool,
    pub r#sch: bool,
    pub r#rog: bool,
    pub r#nin: bool,
    pub r#mch: bool,
    pub r#drk: bool,
    pub r#ast: bool,
    pub r#sam: bool,
    pub r#rdm: bool,
    pub r#blu: bool,
    pub r#gnb: bool,
    pub r#dnc: bool,
    pub r#rpr: bool,
    pub r#sge: bool,
}
impl ClassJobCategory {
    pub fn populate(row: &Row, offset: usize) -> Result<Self, PopulateError> {
        Result::Ok(Self {
            r#name: row.field(0usize + offset)?.into_string()?,
            r#adv: row.field(1usize + offset)?.into_bool()?,
            r#gla: row.field(2usize + offset)?.into_bool()?,
            r#pgl: row.field(3usize + offset)?.into_bool()?,
            r#mrd: row.field(4usize + offset)?.into_bool()?,
            r#lnc: row.field(5usize + offset)?.into_bool()?,
            r#arc: row.field(6usize + offset)?.into_bool()?,
            r#cnj: row.field(7usize + offset)?.into_bool()?,
            r#thm: row.field(8usize + offset)?.into_bool()?,
            r#crp: row.field(9usize + offset)?.into_bool()?,
            r#bsm: row.field(10usize + offset)?.into_bool()?,
            r#arm: row.field(11usize + offset)?.into_bool()?,
            r#gsm: row.field(12usize + offset)?.into_bool()?,
            r#ltw: row.field(13usize + offset)?.into_bool()?,
            r#wvr: row.field(14usize + offset)?.into_bool()?,
            r#alc: row.field(15usize + offset)?.into_bool()?,
            r#cul: row.field(16usize + offset)?.into_bool()?,
            r#min: row.field(17usize + offset)?.into_bool()?,
            r#btn: row.field(18usize + offset)?.into_bool()?,
            r#fsh: row.field(19usize + offset)?.into_bool()?,
            r#pld: row.field(20usize + offset)?.into_bool()?,
            r#mnk: row.field(21usize + offset)?.into_bool()?,
            r#war: row.field(22usize + offset)?.into_bool()?,
            r#drg: row.field(23usize + offset)?.into_bool()?,
            r#brd: row.field(24usize + offset)?.into_bool()?,
            r#whm: row.field(25usize + offset)?.into_bool()?,
            r#blm: row.field(26usize + offset)?.into_bool()?,
            r#acn: row.field(27usize + offset)?.into_bool()?,
            r#smn: row.field(28usize + offset)?.into_bool()?,
            r#sch: row.field(29usize + offset)?.into_bool()?,
            r#rog: row.field(30usize + offset)?.into_bool()?,
            r#nin: row.field(31usize + offset)?.into_bool()?,
            r#mch: row.field(32usize + offset)?.into_bool()?,
            r#drk: row.field(33usize + offset)?.into_bool()?,
            r#ast: row.field(34usize + offset)?.into_bool()?,
            r#sam: row.field(35usize + offset)?.into_bool()?,
            r#rdm: row.field(36usize + offset)?.into_bool()?,
            r#blu: row.field(37usize + offset)?.into_bool()?,
            r#gnb: row.field(38usize + offset)?.into_bool()?,
            r#dnc: row.field(39usize + offset)?.into_bool()?,
            r#rpr: row.field(40usize + offset)?.into_bool()?,
            r#sge: row.field(41usize + offset)?.into_bool()?,
        })
    }
}
