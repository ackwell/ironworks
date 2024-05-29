use std::collections::BTreeSet;

use heck::ToSnakeCase;
use ironworks::file::exh;
use ironworks_schema::{Node, Order, Scalar, Sheet, StructField};
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
struct Context<'a> {
	path: Vec<String>,
	columns: &'a [(usize, exh::ColumnDefinition)],
	items: Vec<TokenStream>,
	uses: BTreeSet<&'static str>,
}

#[derive(Debug)]
struct NodeResult {
	type_: TokenStream,
	reader: TokenStream,
}

/// Generate a rust source code module from a sheet schema definition.
pub fn generate_sheet(sheet: Sheet, columns: Vec<exh::ColumnDefinition>) -> Module {
	if sheet.order == Order::Offset {
		todo!("Offset column order");
	}

	let sheet_name = sheet.name;

	// Run the recursive generation.
	let mut context = Context {
		path: vec![sheet_name.clone()],
		columns: &columns.into_iter().enumerate().collect::<Vec<_>>(),
		items: vec![],
		uses: Default::default(),
	};

	let NodeResult { type_, reader } = generate_node(&mut context, &sheet.node);

	// Extra uses for populator types.
	context.uses.extend([
		"std::result::Result",
		"ironworks::excel::Row",
		"crate::error::PopulateError",
		"crate::metadata::MetadataAdapter",
	]);

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

		impl MetadataAdapter for #type_ {
			fn name() -> String { #sheet_name.to_string() }
			fn populate(row: &Row) -> Result<Self, PopulateError> {
				let offset = 0;
				Result::Ok(#reader)
			}
		}

	  #(#items)*
	};

	Module {
		name: sheet_name.to_snake_case(),
		content: unparse_tokens(file_tokens),
	}
}

fn generate_node(context: &mut Context, node: &Node) -> NodeResult {
	match node {
		Node::Array { count, node } => generate_array(context, count, node),
		Node::Scalar(scalar) => generate_scalar(context, scalar),
		Node::Struct(fields) => generate_struct(context, fields),
	}
}

fn generate_array(context: &mut Context, count: &u32, node: &Node) -> NodeResult {
	// Limit the columns to the first iteration's columns, to avoid any sub-structs
	// trying to generate fields for subsequent indexes.
	// NOTE: This assumes that the schema actually is correct. If it's not, consumers will catch on fire.
	context.columns = context
		.columns
		.get(0..node.size().try_into().unwrap())
		.unwrap_or(&[]);

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

fn generate_scalar(context: &mut Context, _scalar: &Scalar) -> NodeResult {
	// TODO: handle subtypes i.e. references

	let (field_index, column) = match context.columns.get(0) {
		Some(column) => column,
		None => {
			// Definitions include columns that do not exist - represent them as impossible options.
			log::warn!(
				"Path {} resolves to invalid column index.",
				context.path.join("/"),
			);
			context.uses.extend(["std::convert::Infallible"]);
			return NodeResult {
				type_: quote! { Option<Infallible> },
				reader: quote! { None },
			};
		}
	};

	use exh::ColumnKind as K;
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

struct FieldResult {
	identifier: Ident,
	type_: TokenStream,
	reader: TokenStream,
}

fn generate_struct(context: &mut Context, fields: &[StructField]) -> NodeResult {
	let struct_ident = format_ident!("{}", context.path.join("_"));

	let original_columns = context.columns;

	// Walk through the available columns on this struct, generating fields for them.
	let mut field_results = vec![];
	let mut offset = 0u32;
	while offset < u32::try_from(original_columns.len()).unwrap() {
		// TODO: not a fan of this lookup, but... i mean how bad can it be really?
		let field = fields.iter().find(|&field| field.offset == offset);

		// TODO: handle none as an unknown field
		let field_result = match field {
			// A schema field was found - generate a full schema-backed field definition.
			Some(field) => {
				offset += field.node.size();
				generate_struct_field(context, field, original_columns)
			}

			// There's no schema entry for this offset, generate an unknown placeholder field.
			None => {
				let name = format!("unknown{offset}");
				let index = usize::try_from(offset).unwrap();

				offset += 1;

				context.path.push(name.clone());
				context.columns = &original_columns[index..=index];
				let NodeResult { type_, reader } = generate_scalar(context, &Scalar::Default);
				context.path.pop();

				FieldResult {
					identifier: format_ident!("r#{name}"),
					type_,
					reader,
				}
			}
		};

		field_results.push(field_result);
	}

	// There may be fields in the schema that exist outside the valid columns - generate them, letting them fall back to None.
	let mut remaining_fields = fields
		.iter()
		.filter(|field| field.offset >= original_columns.len().try_into().unwrap())
		.map(|field| generate_struct_field(context, field, original_columns))
		.collect::<Vec<_>>();

	field_results.append(&mut remaining_fields);

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

fn generate_struct_field<'a>(
	context: &mut Context<'a>,
	field: &StructField,
	columns: &'a [(usize, exh::ColumnDefinition)],
) -> FieldResult {
	let name_cleaned = sanitize(field.name.clone());
	let identifier = format_ident!("r#{}", name_cleaned.to_snake_case());

	let offset = usize::try_from(field.offset).unwrap();
	let size = usize::try_from(field.node.size()).unwrap();

	context.path.push(name_cleaned);
	context.columns = columns.get(offset..offset + size).unwrap_or(&[]);
	let NodeResult { type_, reader } = generate_node(context, &field.node);
	context.path.pop();

	FieldResult {
		identifier,
		type_,
		reader,
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
