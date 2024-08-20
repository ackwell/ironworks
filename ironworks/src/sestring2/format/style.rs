use crate::sestring2::{error::Result, expression::Expression};

use super::{argument::Arguments, expression::evaluate_expression, format::State};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Style {
	Bold,
	Italic,
	Outline,
	Shadow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorUsage {
	Foreground,
	Edge,
	Shadow,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

pub fn bold<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	style_toggle(Style::Bold, arguments, state)
}

pub fn italic<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	style_toggle(Style::Italic, arguments, state)
}

pub fn edge<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	style_toggle(Style::Outline, arguments, state)
}

pub fn shadow<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	style_toggle(Style::Shadow, arguments, state)
}

fn style_toggle<'a>(style: Style, arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	let enabled = arguments.exhaustive::<bool>(state)?;
	state.writer.set_style(style, enabled)?;
	Ok(())
}

pub fn color_type<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	handle_type(ColorUsage::Foreground, arguments, state)
}

pub fn edge_color_type<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	handle_type(ColorUsage::Edge, arguments, state)
}

fn handle_type<'a>(
	usage: ColorUsage,
	arguments: impl Arguments<'a>,
	state: &mut State,
) -> Result<()> {
	let id = arguments.exhaustive::<u32>(state)?;
	match id {
		0 => state.writer.pop_color(usage)?,
		other => {
			let color = state.input.color(usage, other);
			state.writer.push_color(usage, color)?;
		}
	}

	Ok(())
}

pub fn color<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	handle_color(ColorUsage::Foreground, arguments, state)
}

pub fn edge_color<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	handle_color(ColorUsage::Edge, arguments, state)
}

pub fn shadow_color<'a>(arguments: impl Arguments<'a>, state: &mut State) -> Result<()> {
	handle_color(ColorUsage::Shadow, arguments, state)
}

fn handle_color<'a>(
	usage: ColorUsage,
	arguments: impl Arguments<'a>,
	state: &mut State,
) -> Result<()> {
	let expression = match arguments.exhaustive::<Expression>(state)? {
		// StackColor represents an immediate pop for this usage.
		Expression::StackColor => {
			state.writer.pop_color(usage)?;
			return Ok(());
		}
		other => other,
	};

	let color: u32 = evaluate_expression(expression, state)?.into();
	let [a, r, g, b] = color.to_be_bytes();
	let color = Color { r, g, b, a };
	state.writer.push_color(usage, color)?;

	Ok(())
}
