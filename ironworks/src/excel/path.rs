use super::language::Language;

pub fn exl() -> &'static str {
	"exd/root.exl"
}

pub fn exh(sheet: &str) -> String {
	format!("exd/{sheet}.exh")
}

pub fn exd(sheet: &str, start_id: u32, language: Language) -> String {
	use Language as L;
	let language_suffix = match language {
		L::None => "",
		L::Japanese => "_ja",
		L::English => "_en",
		L::German => "_de",
		L::French => "_fr",
		L::ChineseSimplified => "_chs",
		L::ChineseTraditional => "_cht",
		L::Korean => "_kr",
	};

	format!("exd/{sheet}_{start_id}{language_suffix}.exd")
}
