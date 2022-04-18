use prettyplease::unparse;
use proc_macro2::TokenStream;
use syn::{parse2, File};

pub fn unparse_tokens(tokens: TokenStream) -> String {
	unparse(&parse2::<File>(tokens).unwrap())
}
