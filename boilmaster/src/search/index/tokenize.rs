use std::sync::Arc;

use ironworks::excel;
use lindera_tantivy::{
	dictionary::load_dictionary, tokenizer::LinderaTokenizer, DictionaryConfig, DictionaryKind,
	Mode,
};
use once_cell::sync::OnceCell;
use tantivy::{
	tokenizer::{
		LowerCaser, RawTokenizer, RemoveLongFilter, SimpleTokenizer, Stemmer, TextAnalyzer,
		Tokenizer,
	},
	Index,
};

use crate::data::LanguageString;

// Global wrapper for the lindera tokenizer - by default, the dictionary is
// loaded per analyzer, and with thousands of multilingual sheets leading to
// thousands of instances of the dictionary, the memory usage will quickly OOM
// pretty much any reasonable system without this.
#[derive(Clone)]
struct GlobalLinderaTokenizer(Arc<LinderaTokenizer>);

impl GlobalLinderaTokenizer {
	fn new() -> Self {
		static TOKENIZER: OnceCell<Arc<LinderaTokenizer>> = OnceCell::new();
		let inner = TOKENIZER.get_or_init(|| {
			// TODO: Look into using an external dictionary for lindera so we don't end up bundling the entire thing into the binary. It adds a good 70mb of crap alone.
			let dictionary = load_dictionary(DictionaryConfig {
				kind: Some(DictionaryKind::IPADIC),
				path: None,
			})
			.expect("failed to load tokenization dictionary");
			Arc::new(LinderaTokenizer::new(dictionary, None, Mode::Normal))
		});

		Self(Arc::clone(inner))
	}
}

impl Tokenizer for GlobalLinderaTokenizer {
	fn token_stream<'a>(&self, text: &'a str) -> tantivy::tokenizer::BoxTokenStream<'a> {
		self.0.token_stream(text)
	}
}

pub fn register_tokenizers(index: &Index) {
	let manager = index.tokenizers();

	// TODO: ideally this is done once and shared between all indices? Check what the cost of stuff like the jp dict is
	use excel::Language as EL;
	for language in EL::iter() {
		let name = tokenizer_name(language);
		let tokenizer = match language {
			EL::None => TextAnalyzer::from(RawTokenizer),

			EL::Japanese => TextAnalyzer::from(GlobalLinderaTokenizer::new()),

			EL::English | EL::German | EL::French => european_tokenizer(language),

			// maybe TODO?
			EL::ChineseSimplified => TextAnalyzer::from(RawTokenizer),
			EL::ChineseTraditional => TextAnalyzer::from(RawTokenizer),
			EL::Korean => TextAnalyzer::from(RawTokenizer),
		};

		manager.register(&name, tokenizer)
	}
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
