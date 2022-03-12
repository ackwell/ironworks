use ironworks_ffxiv::SqPackFfxiv;
use ironworks_sqpack::SqPack;

fn main() -> anyhow::Result<()> {
	let sqpack = SqPack::ffxiv()?;

	let file_buffer = sqpack.read_file("exd/root.exl")?;
	let exlt = String::from_utf8(file_buffer)?;

	println!("EXLT: {}", exlt);

	Ok(())
}
