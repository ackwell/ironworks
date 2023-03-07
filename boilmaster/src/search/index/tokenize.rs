use ironworks::excel;
use tantivy::tokenizer::{
	LowerCaser, RawTokenizer, RemoveLongFilter, SimpleTokenizer, Stemmer, TextAnalyzer,
};

use crate::data::LanguageString;

pub fn tokenizers() {
	use excel::Language as EL;

	// TODO: ideally this is done once and shared between all indices? Check what the cost of stuff like the jp dict is
	let foo = EL::iter().map(|language| {
		let name = tokenizer_name(language);
		let tokenizer = match language {
			EL::None => TextAnalyzer::from(RawTokenizer),
			// TODO
			EL::Japanese => TextAnalyzer::from(RawTokenizer),
			EL::English | EL::German | EL::French => european_tokenizer(language),

			// maybe TODO?
			EL::ChineseSimplified => TextAnalyzer::from(RawTokenizer),
			EL::ChineseTraditional => TextAnalyzer::from(RawTokenizer),
			EL::Korean => TextAnalyzer::from(RawTokenizer),
		};

		(name, tokenizer)
	});
}

fn european_tokenizer(language: excel::Language) -> TextAnalyzer {
	use excel::Language as EL;
	use tantivy::tokenizer::Language as TL;

	let stemmer_language = match language {
		EL::English => TL::English,
		EL::German => TL::German,
		EL::French => TL::French,
		_ => panic!("{language:?} cannot be stemmed by european tokenizer pipelines."),
	};

	TextAnalyzer::from(SimpleTokenizer)
		.filter(RemoveLongFilter::limit(40))
		.filter(LowerCaser)
		.filter(Stemmer::new(stemmer_language))
}

pub fn tokenizer_name(language: excel::Language) -> String {
	let language_string = LanguageString::from(language);
	format!("boilmaster_{language_string}")
}
