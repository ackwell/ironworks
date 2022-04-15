use anyhow::Result;
use generate::generate_sheet;
use ironworks_schema::saint_coinach::Provider;

mod generate;

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

	let sheet_name = "CustomTalk";
	let schema = version.sheet(sheet_name)?;

	generate_sheet(sheet_name, schema);

	Ok(())
}
