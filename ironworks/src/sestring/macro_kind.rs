use num_enum::FromPrimitive;

// Based on Lumina implementation as of 2024-08-10
// https://github.com/NotAdam/Lumina/blob/e99b736f18b0962a28d326cd290490b121ed679b/src/Lumina/Text/Payloads/MacroCode.cs

/// Macro functions that may be used in a
/// [payload](super::payload::MacroPayload).
///
/// Each macro has discrete behavior and expected arguments. Check
/// [`format`](super::format) for examples of implemented behavior.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum MacroKind {
	SetResetTime = 0x06,
	SetTime = 0x07,
	If = 0x08,
	Switch = 0x09,
	PcName = 0x0A,
	IfPcGender = 0x0B,
	IfPcName = 0x0C,
	Josa = 0x0D,
	Josaro = 0x0E,
	IfSelf = 0x0F,
	NewLine = 0x10,
	Wait = 0x11,
	Icon = 0x12,
	Color = 0x13,
	EdgeColor = 0x14,
	ShadowColor = 0x15,
	SoftHyphen = 0x16,
	Key = 0x17,
	Scale = 0x18,
	Bold = 0x19,
	Italic = 0x1A,
	Edge = 0x1B,
	Shadow = 0x1C,
	NonBreakingSpace = 0x1D,
	Icon2 = 0x1E,
	Hyphen = 0x1F,
	Num = 0x20,
	Hex = 0x21,
	Kilo = 0x22,
	Byte = 0x23,
	Sec = 0x24,
	Time = 0x25,
	Float = 0x26,
	Link = 0x27,
	Sheet = 0x28,
	String = 0x29,
	Caps = 0x2A,
	Head = 0x2B,
	Split = 0x2C,
	HeadAll = 0x2D,
	Fixed = 0x2E,
	Lower = 0x2F,
	JaNoun = 0x30,
	EnNoun = 0x31,
	DeNoun = 0x32,
	FrNoun = 0x33,
	ChNoun = 0x34,
	LowerHead = 0x40,
	ColorType = 0x48,
	EdgeColorType = 0x49,
	Ruby = 0x4A,
	Digit = 0x50,
	Ordinal = 0x51,
	Sound = 0x60,
	LevelPos = 0x61,

	#[num_enum(catch_all)]
	Unknown(u8),
}
