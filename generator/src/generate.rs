use ironworks_schema::{Node, ReferenceTarget, Sheet};
use lazy_static::lazy_static;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use regex::Regex;

#[derive(Debug)]
struct Context {
	path: Vec<String>,
	items: Vec<TokenStream>,
}

// TODO: some note about being an entry point
// TODO: I'm not entirely convinced by passing the sheet name in here...
pub fn generate_sheet(name: &str, sheet: Sheet) {
	let mut context = Context {
		path: vec![name.into()],
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
	let count = usize::try_from(*count).unwrap();
	quote! { [#type_identifier; #count] }
}

fn generate_reference(_context: &mut Context, _targets: &[ReferenceTarget]) -> TokenStream {
	let identifier = format_ident!("TodoReference");
	quote! { #identifier }
}

fn generate_scalar(_context: &mut Context) -> TokenStream {
	// TODO: this will need column header data to resolve the type
	let identifier = format_ident!("TodoScalar");
	quote! { #identifier }
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
