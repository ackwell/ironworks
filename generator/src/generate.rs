use ironworks::excel::{Column, ColumnKind};
use ironworks_schema::{Node, Order, ReferenceTarget, Sheet};
use lazy_static::lazy_static;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;

// TODO: can probably make a decent chunk of this instance methods on &mut self of the context
#[derive(Debug)]
struct Context {
	path: Vec<String>,
	columns: Vec<Column>,
	column_index: usize,
	items: Vec<TokenStream>,
}

// TODO: some note about being an entry point
// TODO: I'm not entirely convinced by passing the sheet name in here...
pub fn generate_sheet(name: &str, sheet: Sheet, columns: Vec<Column>) {
	if sheet.order == Order::Offset {
		todo!("Offset column order");
	}

	let mut context = Context {
		path: vec![name.into()],
		columns,
		column_index: 0,
		items: vec![],
	};

	// TODO: handle the order field
	generate_node(&mut context, &sheet.node);

	let items = context.items;

	let file_tokens = quote! {
	  #(#items)*
	};

	let file_tree = syn::parse2::<syn::File>(file_tokens).unwrap();
	let formatted = prettyplease::unparse(&file_tree);

	println!("{formatted}")
}

// TODO: gen node should probably return the (rust) type of the node
// TODO: it'll also need to return some way to _read_ itself - or is that a context thing? nah?
fn generate_node(context: &mut Context, node: &Node) -> TokenStream {
	match node {
		Node::Array { count, node } => generate_array(context, count, node),
		Node::Reference(targets) => generate_reference(context, targets),
		Node::Scalar => generate_scalar(context),
		Node::Struct(fields) => generate_struct(context, fields),
	}
}

fn generate_array(context: &mut Context, count: &u32, node: &Node) -> TokenStream {
	let type_identifier = generate_node(context, node);

	// Walking the array's node will have advanced the column index equivalent to
	// a count of 1 - skip over any remaining count to ensure further lookups
	// resume from the right spot.
	// NOTE: This assumes the array count is correct.
	context.column_index += usize::try_from(node.size() * (count - 1)).unwrap();

	let count = usize::try_from(*count).unwrap();
	quote! { [#type_identifier; #count] }
}

fn generate_reference(context: &mut Context, _targets: &[ReferenceTarget]) -> TokenStream {
	// TODO: should i try to make references work as a superset of scalars?
	let column = &context.columns[context.column_index];
	context.column_index += 1;

	let temp = format!("{:#?}", column.kind());
	let identifier = format_ident!("TodoReference_{temp}");

	quote! { #identifier }
}

fn generate_scalar(context: &mut Context) -> TokenStream {
	let column = &context.columns[context.column_index];
	context.column_index += 1;

	let scalar_type = to_type(column.kind());

	quote! { #scalar_type }
}

fn generate_struct(context: &mut Context, fields: &[(String, Node)]) -> TokenStream {
	// TODO: actually make this properly
	let struct_ident = format_ident!("{}", context.path.join("_"));

	let field_tokens = fields.iter().map(|(name, node)| {
		let name_identifier = to_identifier(name);
		// TODO: this will need to push->pop the name ident onto the path? I think?
		let type_identifier = generate_node(context, node);
		quote! { #name_identifier: #type_identifier }
	});

	let struct_tokens = quote! {
		struct #struct_ident {
			#(#field_tokens),*
		}
	};

	context.items.push(struct_tokens);

	quote! { #struct_ident }
}

lazy_static! {
	static ref RE_INVALID_CHARS: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
}

// TODO: This might be better off as a -> Cow<str> "sanitize" function so we can sanitize the path before it becomes an ident
fn to_identifier(arg: &str) -> Ident {
	let sanitized = RE_INVALID_CHARS.replace_all(arg, "");
	format_ident!("{sanitized}")
}

fn to_type(kind: ColumnKind) -> TokenStream {
	use ColumnKind as K;

	// TODO: might need a second similar match statement for read logic on scalars - do i combine the two?
	match kind {
		K::String => quote! { String },

		K::Bool
		| K::PackedBool0
		| K::PackedBool1
		| K::PackedBool2
		| K::PackedBool3
		| K::PackedBool4
		| K::PackedBool5
		| K::PackedBool6
		| K::PackedBool7 => quote! { bool },

		K::Int8 => quote! { i8 },
		K::Int16 => quote! { i16 },
		K::Int32 => quote! { i32 },
		K::Int64 => quote! { i64 },

		K::UInt8 => quote! { u8 },
		K::UInt16 => quote! { u16 },
		K::UInt32 => quote! { u32 },
		K::UInt64 => quote! { u64 },

		K::Float32 => quote! { f32 },
	}
}
