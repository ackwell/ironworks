use std::borrow::Cow;

use anyhow::Result;
use ironworks_schema::{saint_coinach::Provider, Node, Sheet};
use lazy_static::lazy_static;
use quote::{format_ident, quote};
use regex::Regex;

fn main() -> Result<()> {
	saint_coinach()?;

	Ok(())
}

// TODO: Seperate file and all that jazz.
fn saint_coinach() -> Result<()> {
	let provider = Provider::new()?;
	// TODO: fetch updates to the provider to make sure it's fresh
	// TODO: arg for version?
	let version = provider.version("HEAD")?;

	let schema = version.schema("CustomTalk")?;

	generate_sheet(schema);

	Ok(())
}

// TODO: some note about being an entry point
// TOOD: i'll probably need some generator "context" concept to register structs onto and suchforth
// TODO: names travel down the tree - how do i name the structs I generate? context?
fn generate_sheet(sheet: Sheet) {
	// TODO: handle the order field
	generate_node(sheet.schema);
}

fn generate_node(node: Node) {
	match node {
		Node::Array { .. } => todo!("array"),
		Node::Reference(_) => todo!("reference"),
		Node::Scalar => todo!("scalar"),
		Node::Struct(fields) => generate_struct(fields),
	}
}

fn generate_struct(fields: Vec<(String, Node)>) {
	let bar = fields.iter().map(|(name, _)| {
		let name_identifier = format_ident!("{}", something(name));
		quote! { #name_identifier: Todo }
	});

	let foo = quote! {
		struct Test {
			#(#bar),*
		}
	};

	println!("{foo}")
}

// ???
lazy_static! {
	static ref RE_INVALID_CHARS: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
}

fn something(arg: &str) -> Cow<str> {
	RE_INVALID_CHARS.replace_all(arg, "")
}
