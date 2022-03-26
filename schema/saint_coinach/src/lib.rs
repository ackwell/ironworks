use std::{
	collections::HashMap,
	env::current_exe,
	fmt::Display,
	hash::Hash,
	path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, Commit, Object, Repository};
use ironworks_schema_core::Node;
use lazy_static::lazy_static;
use serde_json::Value;

// need to build some trait that represents what a "schema provider" looks like (ref manacutter probably)
// impl of that trait for stc can probably own a repository ref and do lazy lookups into the object db
// given how git works a canonical version is easy to trip a "need to update" check but will need to consider how to trip an update check for a ref like "HEAD"

// Default configuration
const REPOSITORY_URL: &str = "https://github.com/xivapi/SaintCoinach.git";
const REPOSITORY_DIRECTORY: &str = "saint_coinach";

lazy_static! {
	static ref DEFINITION_PATH: PathBuf = ["SaintCoinach", "Definitions"].iter().collect();
}

#[derive(thiserror::Error, Debug)]
enum Error {
	// TODO: I should probably make the not found errors more data-y, like _what_ wasn't found _where_, etc.
	#[error("Not found: {0}")]
	NotFound(String),

	// TODO: This exposes the fact that we _use_ git, but not the impl details of git2. is that enough? is that too much? I'm not sure.
	#[error("Repository error: {0}")]
	Repository(String),

	#[error("Schema error: {0}")]
	Schema(String),
}

// TODO: aaaaaa i don't knoooow. if kept, doc(hidden)?
impl From<git2::Error> for Error {
	fn from(error: git2::Error) -> Self {
		Error::Repository(error.to_string())
	}
}

type Result<T, E = Error> = std::result::Result<T, E>;

// todo: name?
#[derive(Debug)]
struct SaintCoinachSchemaOptions {
	remote: Option<String>,
	directory: Option<PathBuf>,
}

impl SaintCoinachSchemaOptions {
	fn new() -> Self {
		SaintCoinachSchemaOptions {
			remote: None,
			directory: None,
		}
	}

	fn remote(&mut self, remote: impl ToString) -> &mut Self {
		self.remote = Some(remote.to_string());
		self
	}

	fn directory(&mut self, directory: impl Into<PathBuf>) -> &mut Self {
		self.directory = Some(directory.into());
		self
	}

	#[inline]
	fn build(&self) -> Result<SaintCoinachSchema> {
		SaintCoinachSchema::with_options(self)
	}
}

// TODO: can't derive debug on this due to repo - look into crates like `derivative` to handle?
struct SaintCoinachSchema {
	repository: Repository,
}

impl SaintCoinachSchema {
	#[inline]
	fn new() -> Result<Self> {
		Self::with_options(&Self::options())
	}

	#[inline]
	fn options() -> SaintCoinachSchemaOptions {
		SaintCoinachSchemaOptions::new()
	}

	fn with_options(options: &SaintCoinachSchemaOptions) -> Result<Self> {
		// todo: look into fs::canonicalize but it sounds like it only works for pre-existing stuff
		let directory = options
			.directory
			.clone()
			.or_else(default_directory)
			.ok_or_else(|| {
				Error::NotFound(
					"No directory was provided, and default directory could not be resolved."
						.to_string(),
				)
			})?;

		let remote = options
			.remote
			.clone()
			.unwrap_or_else(|| REPOSITORY_URL.to_string());

		let repository = if directory.exists() {
			let repository = Repository::open_bare(&directory)?;
			// If the pre-existing repository points to an origin we didn't expect,
			// fail out now so it doesn't do something weird later.
			match repository.find_remote("origin")?.url() {
				Some(url) if url == remote => (),
				url => {
					return Err(Error::Repository(format!(
						"Repository at {:?} has origin {}, expected {}.",
						&directory,
						url.unwrap_or("(none)"),
						remote
					)))
				}
			}

			log::trace!("Opened SaintCoinach at {:?}", directory);
			repository
		} else {
			log::info!("Cloning SaintCoinach from {} to {:?}", remote, directory);
			RepoBuilder::new().bare(true).clone(&remote, &directory)?
		};

		Ok(Self { repository })
	}

	fn version(&self, spec: &str) -> Result<SaintCoinachVersion> {
		let commit = self.repository.revparse_single(spec)?.peel_to_commit()?;
		Ok(SaintCoinachVersion {
			repository: &self.repository,
			commit,
		})
	}
}

fn default_directory() -> Option<PathBuf> {
	match current_exe() {
		Ok(path) => path
			.parent()
			.map(|parent| parent.join(REPOSITORY_DIRECTORY)),
		Err(_) => None,
	}
}

// this should impl a "version" trait or something
struct SaintCoinachVersion<'repo> {
	// Should we be Rc-ing the repo so versions can live seperately? Not sure how the lifetime on the commit would work there.
	repository: &'repo Repository,
	commit: Commit<'repo>,
}

impl SaintCoinachVersion<'_> {
	// thoughts; for hash map keying & stuff
	fn id(&self) -> impl Eq + Hash + Display {
		self.commit.id()
	}

	// fn schemas -> iter

	fn schema(&self, sheet: &str) -> Result<()> {
		let path = DEFINITION_PATH.join(format!("{}.json", sheet));

		let object = self
			.object_at_path(&path)
			.map_err(|error| match error.code() {
				git2::ErrorCode::NotFound => {
					Error::NotFound(format!("Definition for sheet {}", sheet))
				}
				_ => Error::from(error),
			})?;

		let blob = object.as_blob().ok_or_else(|| {
			Error::Repository(format!(
				"Expected blob for {} sheet schema, got {:?}",
				sheet,
				object.kind()
			))
		})?;

		println!("{}", String::from_utf8_lossy(blob.content()));

		let foo = serde_json::from_slice::<Value>(blob.content());
		let def = read_sheet_definition(&foo.unwrap());

		println!("def {:#?}", def);

		Ok(())
	}

	fn object_at_path(&self, path: &Path) -> Result<Object<'_>, git2::Error> {
		self.commit
			.tree()?
			.get_path(path)?
			.to_object(self.repository)
	}
}

/// See also:
/// - [SheetDefinition.cs#L157](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/Definition/SheetDefinition.cs#L157)
/// - [PositionedDataDefinition.cs#L71](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/Definition/PositionedDataDefinition.cs#L71)
fn read_sheet_definition(value: &Value) -> Result<Node> {
	let mut nodes = HashMap::<String, (u32, Node)>::new();

	let definitions = value.get("definitions").and_then(Value::as_array);
	for definition in definitions.iter().flat_map(|values| *values) {
		// PositionedDataDefinition inlined as it's only used in one location, and makes setting up the struct fields simpler
		let index = definition
			.get("index")
			.and_then(|value| value.as_u64())
			.unwrap_or(0);

		// do we want to retun name? what's that about
		// TODO: This effectively shortcuts the entire read if an error bubbles up - is that the behavior we want? probably?
		let (node, name) = read_data_definition(definition)?;

		nodes.insert(
			name.unwrap_or_else(|| format!("Unnamed{}", index)),
			(index.try_into().unwrap(), node),
		);
	}

	Ok(Node::Struct(nodes))
}

/// See also:
/// - [IDataDefinition.cs#L34](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/Definition/IDataDefinition.cs#L34)
fn read_data_definition(value: &Value) -> Result<(Node, Option<String>)> {
	match value.get("type").and_then(Value::as_str) {
		None => read_single_data_definition(value),
		Some("group") => read_group_data_definition(value),
		Some("repeat") => read_repeat_data_definition(value),
		Some(unknown) => Err(Error::Schema(format!("Unknown data type {}", unknown))),
	}
}

/// See also:
/// - [SingleDataDefinition.cs#L66](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/Definition/SingleDataDefinition.cs#L66)
fn read_single_data_definition(value: &Value) -> Result<(Node, Option<String>)> {
	let name = value.get("name").and_then(Value::as_str).map(String::from);

	let converter = match value.get("converter") {
		Some(object) => object,
		None => return Ok((Node::Scalar, name)),
	};

	// TODO: There's also a "quad" type with a converter but I've got no idea how it's instantiated.
	let node = match converter.get("type").and_then(Value::as_str) {
		Some("color") => read_color_converter(converter),
		Some("generic") => read_generic_reference_converter(converter),
		Some("icon") => read_icon_converter(converter),
		Some("multiref") => read_multi_reference_converter(converter),
		Some("link") => read_sheet_link_converter(converter),
		Some("tomestone") => read_tomestone_or_item_reference_converter(converter),
		Some("complexlink") => read_complex_link_converter(converter),
		unknown => Err(Error::Schema(format!(
			"Unknown converter type {}",
			unknown.unwrap_or("(none)")
		))),
	};

	Ok((node?, name))
}

/// See also:
/// - [GroupDataDefinition.cs#L125](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/Definition/GroupDataDefinition.cs#L125)
fn read_group_data_definition(value: &Value) -> Result<(Node, Option<String>)> {
	let members = value.get("members").and_then(Value::as_array);
	let nodes = members
		.iter()
		.flat_map(|members| *members)
		.scan(0u32, |size, member| {
			Some(read_data_definition(member).map(|(node, name)| {
				let current_size = *size;
				*size += node.size();

				(
					name.unwrap_or_else(|| format!("Unnamed{}", current_size)),
					(current_size, node),
				)
			}))
		})
		.collect::<Result<HashMap<_, _>>>()?;

	Ok((Node::Struct(nodes), None /* TODO */))
}

/// See also:
/// - [RepeatDataDefinition.cs#L85](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/Definition/RepeatDataDefinition.cs#L85)
fn read_repeat_data_definition(value: &Value) -> Result<(Node, Option<String>)> {
	// TODO: These... as well as all the other errors, really... have no way to pinpoint _where_ the error occured. Look into it.
	let definition = value
		.get("definition")
		.ok_or_else(|| Error::Schema("Repeat missing definition".to_string()))?;

	let count = value
		.get("count")
		.and_then(Value::as_u64)
		.ok_or_else(|| Error::Schema("Repeat missing count".to_string()))?;

	let (node, name) = read_data_definition(definition)?;

	Ok((Node::Array(count.try_into().unwrap(), Box::new(node)), name))
}

/// See also:
/// - [ColorConverter.cs#L46](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/ValueConverters/ColorConverter.cs#L46)
fn read_color_converter(_value: &Value) -> Result<Node> {
	// TODO: ?
	Ok(Node::Scalar)
}

/// See also:
/// - [GenericReferenceConverter.cs#L33](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/ValueConverters/GenericReferenceConverter.cs#L33)
fn read_generic_reference_converter(_value: &Value) -> Result<Node> {
	// TODO: ?
	Ok(Node::Scalar)
}

/// See also:
/// - [IconConverter.cs#L33](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/ValueConverters/IconConverter.cs#L33)
fn read_icon_converter(_value: &Value) -> Result<Node> {
	// TODO: ?
	Ok(Node::Scalar)
}

/// See also:
/// - [MultiReferenceConverter.cs#L50](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/ValueConverters/MultiReferenceConverter.cs#L50)
fn read_multi_reference_converter(_value: &Value) -> Result<Node> {
	// TODO: This should be a reference node, once I add those.
	Ok(Node::Scalar)
}

/// See also:
/// - [SheetLinkConverter.cs#L40](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/ValueConverters/SheetLinkConverter.cs#L40)
fn read_sheet_link_converter(_value: &Value) -> Result<Node> {
	// TODO: Likewise should be a reference
	Ok(Node::Scalar)
}

/// See also:
/// - [TomestoneOrItemReferenceConverter.cs#L54](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/ValueConverters/TomestoneOrItemReferenceConverter.cs#L54)
fn read_tomestone_or_item_reference_converter(_value: &Value) -> Result<Node> {
	// TODO: ?
	Ok(Node::Scalar)
}

/// See also:
/// - [ComplexLinkConverter.cs#L143](https://github.com/xivapi/SaintCoinach/blob/800eab3e9dd4a2abc625f53ce84dad24c8579920/SaintCoinach/Ex/Relational/ValueConverters/ComplexLinkConverter.cs#L143)
fn read_complex_link_converter(_value: &Value) -> Result<Node> {
	// TODO: Likewise should be a reference
	Ok(Node::Scalar)
}

pub fn test() {
	let schema = SaintCoinachSchema::new().unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089fad484c25e491d3c37").unwrap();
	// let version = schema.version("69caa7e14fed1caaeb2089").unwrap();
	// let version = schema.version("refs/tags/69caa7e").unwrap();
	let version = schema.version("HEAD").unwrap();
	// let version = schema.version("master").unwrap();

	version.schema("RelicNote").unwrap();
}
