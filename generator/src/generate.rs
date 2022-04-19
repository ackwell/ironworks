use std::{borrow::Cow, collections::HashSet};

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
	// TODO: Should this be an Ident?
	pub name: String,
	// TODO: Should this be a token stream?
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

// TODO: some note about being an entry point
pub fn generate_sheet(sheet: Sheet, columns: Vec<Column>) -> Module {
	if sheet.order == Order::Offset {
		todo!("Offset column order");
	}

	let mut context = Context {
		path: vec![sheet.name.clone()],
		columns,
		column_index: 0,
		items: vec![],
		uses: Default::default(),
	};

	generate_node(&mut context, &sheet.node);

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
	]);

	let type_ = quote! { Vec<#identifier> };

	NodeResult {
		reader: quote! {
			(0..#count_usize)
				.map(|index| {
					let offset = offset + #node_size * index;
					Result::Ok(#reader)
				})
				.collect::<Result<#type_, PopulateError>>()?
		},
		type_,
	}
}

fn generate_reference(context: &mut Context, _targets: &[ReferenceTarget]) -> NodeResult {
	// TODO: reference logic
	generate_scalar(context)
}

fn generate_scalar(context: &mut Context) -> NodeResult {
	let column = &context.columns[context.column_index];
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

	// TODO: Should possibly put the col idx offset and field idens as statics or something so it's consistent.
	NodeResult {
		type_: quote! { #scalar_type },
		reader: quote! { row.field(#field_index + offset)?.#converter()? },
	}
}

fn generate_struct(context: &mut Context, fields: &[(String, Node)]) -> NodeResult {
	// TODO: actually make this properly
	let struct_ident = format_ident!("{}", context.path.join("_"));

	struct FieldResult {
		identifier: Ident,
		type_: TokenStream,
		reader: TokenStream,
	}

	let field_results = fields
		.iter()
		.map(|(name, node)| {
			let identifier = format_ident!("{}", sanitize(name).to_snake_case());

			// TODO: this will need to push->pop the name ident onto the path? I think?
			context.path.push(name.clone());
			let NodeResult { type_, reader } = generate_node(context, node);
			context.path.pop();

			FieldResult {
				identifier,
				type_,
				reader,
			}
		})
		.collect::<Vec<_>>();

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
			/// todo docs will probably need to build outside
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
		// TODO: do we need to fully qualify the ident here?
		reader: quote! { #struct_ident::populate(row, offset)? },
	}
}

lazy_static! {
	static ref RE_INVALID_CHARS: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
}

// TODO: This might be better off as a -> Cow<str> "sanitize" function so we can sanitize the path before it becomes an ident
fn sanitize(arg: &str) -> Cow<str> {
	RE_INVALID_CHARS.replace_all(arg, "")
}
