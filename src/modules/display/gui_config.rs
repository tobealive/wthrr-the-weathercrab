use colored::{Color, ColoredString, Colorize};
use optional_struct::{optional_struct, Applyable};
use serde::{Deserialize, Serialize};

use crate::modules::display::{
	border::BorderStyle,
	graph::{ConfigFileGraphOpts, GraphOpts},
};

#[optional_struct(ConfigFileGui)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Gui {
	pub border: BorderStyle,
	pub color: ColorVariant,
	#[optional_rename(ConfigFileGraphOpts)]
	pub graph: GraphOpts,
	pub greeting: bool,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ColorVariant {
	#[default]
	default,
	plain,
}

impl Default for Gui {
	fn default() -> Self {
		Self {
			border: BorderStyle::default(),
			color: ColorVariant::default,
			graph: GraphOpts::default(),
			greeting: true,
		}
	}
}

pub trait ColorOption {
	fn color_option(self, default_color: Color, config_color: &ColorVariant) -> ColoredString;
}

impl<'a> ColorOption for &'a str {
	fn color_option(self, default_color: Color, config_color: &ColorVariant) -> ColoredString {
		match config_color {
			ColorVariant::plain => self.normal(),
			ColorVariant::default => self.color(default_color),
		}
	}
}
