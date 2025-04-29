//! Structs and utilities for parsing ZiPatch .patch files.

mod chunk;
mod command;
mod patch;

pub use {
	chunk::{
		AddDirectoryChunk, ApplyChunk, Chunk, ChunkContainer, DeleteDirectoryChunk,
		FileHeaderChunk, FileHeaderChunkV3, OptionKind, SqPackChunk,
	},
	command::{
		AddCommand, BlockHeader, DeleteCommand, ExpandCommand, FileOperation, FileOperationCommand,
		HeaderFileKind, HeaderKind, HeaderUpdateCommand, IndexUpdateCommand, IndexUpdateKind,
		PatchInfoCommand, SqPackFile, TargetInfoCommand, TargetPlatform, TargetRegion,
	},
	patch::Header,
};
