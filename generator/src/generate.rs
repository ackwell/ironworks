use std::collections::HashSet;

use heck::ToSnakeCase;
use ironworks::excel::{Column, ColumnKind};
use ironworks_schema::{Node, Order, ReferenceTarget, Sheet};
use lazy_static::lazy_static;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;

use crate::utility::unparse_tokens;

#[derive(Debug)]
pub struct Module {
	pub name: String,
	pub content: String,
}

#[derive(Debug)]
struct Context {
	path: Vec<String>,
	columns: Vec<Column>,
	column_index: usize,
	items: Vec<TokenStream>,
	uses: HashSet<&'static str>,
}

#[derive(Debug)]
struct NodeResult {
	type_: TokenStream,
	reader: TokenStream,
}

/// Generate a rust source code module from a sheet schema definition.
pub fn generate_sheet(sheet: Sheet, columns: Vec<Column>) -> Module {
	if sheet.order == Order::Offset {
		todo!("Offset column order");
	}

	// Run the recursive generation.
	let mut context = Context {
		path: vec![sheet.name.clone()],
		columns,
		column_index: 0,
		items: vec![],
		uses: Default::default(),
	};

	generate_node(&mut context, &sheet.node);

	// Collate results into a token stream.
	let uses = context
		.uses
		.iter()
		.map(|string| str::parse::<TokenStream>(string))
		.collect::<Result<Vec<_>, _>>()
		.unwrap();
	let items = context.items;
	let file_tokens = quote! {
		#(use #uses;)*

	  #(#items)*
	};

	Module {
		name: sheet.name.to_snake_case(),
		content: unparse_tokens(file_tokens),
	}
}

fn generate_node(context: &mut Context, node: &Node) -> NodeResult {
	match node {
		Node::Array { count, node } => generate_array(context, count, node),
		Node::Reference(targets) => generate_reference(context, targets),
		Node::Scalar => generate_scalar(context),
		Node::Struct(fields) => generate_struct(context, fields),
	}
}

fn generate_array(context: &mut Context, count: &u32, node: &Node) -> NodeResult {
	let NodeResult {
		type_: identifier,
		reader,
	} = generate_node(context, node);

	// Walking the array's node will have advanced the column index equivalent to
	// a count of 1 - skip over any remaining count to ensure further lookups
	// resume from the right spot.
	// NOTE: This assumes the array count is correct.
	let count_usize = usize::try_from(*count).unwrap();
	let node_size = usize::try_from(node.size()).unwrap();
	context.column_index += node_size * (count_usize - 1);

	context.uses.extend([
		"std::result::Result",
		"std::vec::Vec",
		"crate::error::PopulateError",
		"crate::utility::read_array",
	]);

	let type_ = quote! { Vec<#identifier> };

	NodeResult {
		reader: quote! {
			read_array(offset, #count_usize, #node_size, |offset| {
				Result::Ok(#reader)
			})?
		},
		type_,
	}
}

fn generate_reference(context: &mut Context, _targets: &[ReferenceTarget]) -> NodeResult {
	// TODO: reference logic
	generate_scalar(context)
}

fn generate_scalar(context: &mut Context) -> NodeResult {
	let column = match context.columns.get(context.column_index) {
		Some(column) => column,
		None => {
			// Definitions include columns that do not exist - represent them as impossible options.
			context.uses.extend(["std::convert::Infallible"]);
			return NodeResult {
				type_: quote! { Option<Infallible> },
				reader: quote! { None },
			};
		}
	};

	context.column_index += 1;

	let field_index = column.index();

	use ColumnKind as K;
	let (scalar_type, converter) = match column.kind() {
		K::String => {
			context.uses.extend(["ironworks::sestring::SeString"]);
			(quote! { SeString }, quote! { into_string })
		}

		K::Bool
		| K::PackedBool0
		| K::PackedBool1
		| K::PackedBool2
		| K::PackedBool3
		| K::PackedBool4
		| K::PackedBool5
		| K::PackedBool6
		| K::PackedBool7 => (quote! { bool }, quote! { into_bool }),

		K::Int8 => (quote! { i8 }, quote! { into_i8 }),
		K::Int16 => (quote! { i16 }, quote! { into_i16 }),
		K::Int32 => (quote! { i32 }, quote! { into_i32 }),
		K::Int64 => (quote! { i64 }, quote! { into_i64 }),

		K::UInt8 => (quote! { u8 }, quote! { into_u8 }),
		K::UInt16 => (quote! { u16 }, quote! { into_u16 }),
		K::UInt32 => (quote! { u32 }, quote! { into_u32 }),
		K::UInt64 => (quote! { u64 }, quote! { into_u64 }),

		K::Float32 => (quote! { f32 }, quote! { into_f32 }),
	};

	NodeResult {
		type_: quote! { #scalar_type },
		reader: quote! { row.field(#field_index + offset)?.#converter()? },
	}
}

fn generate_struct(context: &mut Context, fields: &[(String, Node)]) -> NodeResult {
	let struct_ident = format_ident!("{}", context.path.join("_"));

	// Walk fields to build the reading logic for them.
	struct FieldResult {
		identifier: Ident,
		type_: TokenStream,
		reader: TokenStream,
	}

	let field_results = fields
		.iter()
		.map(|(name, node)| {
			let name_cleaned = sanitize(name.clone());
			// TODO: ident parse will err on keyword, use that to avoid prefix?
			let identifier = format_ident!("r#{}", name_cleaned.to_snake_case());

			context.path.push(name_cleaned);
			let NodeResult { type_, reader } = generate_node(context, node);
			context.path.pop();

			FieldResult {
				identifier,
				type_,
				reader,
			}
		})
		.collect::<Vec<_>>();

	// Build out the containing struct.
	let identifiers = field_results
		.iter()
		.map(|result| &result.identifier)
		.collect::<Vec<_>>();
	let types = field_results.iter().map(|result| &result.type_);
	let readers = field_results.iter().map(|result| &result.reader);

	context.uses.extend([
		"std::result::Result",
		"ironworks::excel::Row",
		"crate::error::PopulateError",
	]);

	let struct_tokens = quote! {
		#[derive(Debug)]
		pub struct #struct_ident {
			#(pub #identifiers: #types),*
		}

		// TODO: tempted to make this an `impl Populator` or something, and provide a default impl fn that automates the offset &c
		impl #struct_ident {
			pub fn populate(
				row: &Row,
				offset: usize,
			) -> Result<Self, PopulateError> {
				Result::Ok(Self {
					#(#identifiers: #readers),*
				})
			}
		}
	};

	context.items.push(struct_tokens);

	NodeResult {
		type_: quote! { #struct_ident },
		reader: quote! { #struct_ident::populate(row, offset)? },
	}
}

lazy_static! {
	static ref RE_INVALID_CHARS: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
}

const NUMBERS: &[&str] = &[
	"Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

fn sanitize(arg: String) -> String {
	let mut out = arg;

	// If there's a leading digit in the string, replace it with a textual representation.
	let leading_digit = out
		.chars()
		.next()
		.and_then(|char| char.to_digit(10).map(|digit| (digit, char.len_utf8())));
	if let Some((digit, len)) = leading_digit {
		out = format!(
			"{}{}",
			NUMBERS[usize::try_from(digit).unwrap()],
			out.split_at(len).1
		);
	};

	// Replace common symbols with meaningful info.
	out = out.replace('%', "Percent");

	// Remove any remaining invalid characters.
	RE_INVALID_CHARS.replace_all(&out, "").into_owned()
}
