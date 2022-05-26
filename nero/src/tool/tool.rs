use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use strum::{EnumIter, IntoEnumIterator};

use super::{dev::DevTool, explorer::ExplorerTool};

#[derive(Clone, Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum Tool {
	Dev,
	Explorer,
}

// TODO: work out how to move this metadata into the tools themselves?
impl Tool {
	pub fn name(&self) -> String {
		#[allow(clippy::match_single_binding)]
		match self {
			other => format!("{other:?}"),
		}
	}

	pub fn icon(&self) -> &'static str {
		match self {
			Self::Dev => "icon-code.png",
			Self::Explorer => "icon-folder-tree.png",
		}
	}
}

pub struct ToolPlugins;
impl Plugin for ToolPlugins {
	fn build(&self, app: &mut App) {
		app.add_loopless_state(Tool::iter().next().unwrap())
			.add_plugin(DevTool)
			.add_plugin(ExplorerTool);
	}
}
