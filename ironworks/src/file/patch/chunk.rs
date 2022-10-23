use binrw::binread;

#[binread]
#[br(big)]
#[derive(Debug)]
pub enum Chunk {
	#[br(magic = b"FHDR")]
	FileHeader,

	#[br(magic = b"APLY")]
	Apply,

	#[br(magic = b"ADIR")]
	AddDirectory,

	#[br(magic = b"DELD")]
	DeleteDirectory,

	#[br(magic = b"SQPK")]
	SqPack,

	#[br(magic = b"EOF_")]
	EndOfFile,
}
