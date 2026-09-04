//! The `Cxxx` items of a timeline: the instructions a track runs.

use std::io::{Read, Seek};

use binrw::{BinRead, BinResult, Endian, binread};
use getset::CopyGetters;

use super::{at_list, at_offset, offset_floats, offset_string, rest};

/// One `Cxxx` item.
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct Command {
	id: i16,

	/// When the command runs, in the same units as the timeline's duration.
	time: i16,

	#[getset(skip)]
	kind: CommandKind,
}

impl Command {
	/// What the command does.
	pub fn kind(&self) -> &CommandKind {
		&self.kind
	}

	pub(super) fn parse<R: Read + Seek>(
		reader: &mut R,
		endian: Endian,
		magic: &[u8; 4],
		base: u64,
		end: u64,
	) -> BinResult<Self> {
		let id = i16::read_options(reader, endian, ())?;
		let time = i16::read_options(reader, endian, ())?;
		let kind = match kind(reader, endian, magic, base)? {
			Some(kind) => kind,
			None => CommandKind::Unknown {
				magic: *magic,
				body: rest(reader, end)?,
			},
		};
		Ok(Self { id, time, kind })
	}
}

/// One field of a command body.
trait Field: Sized {
	/// `base` is the item's offset base, `item_start + 8`.
	fn read<R: Read + Seek>(reader: &mut R, endian: Endian, base: u64) -> BinResult<Self>;
}

macro_rules! inline {
	($($type:ty)*) => {
		$(impl Field for $type {
			fn read<R: Read + Seek>(reader: &mut R, endian: Endian, _base: u64) -> BinResult<Self> {
				<$type as BinRead>::read_options(reader, endian, ())
			}
		})*
	};
}

inline!(u8 i16 u16 i32 u32 f32 [f32; 2] [f32; 3] [f32; 4] [u32; 17]);

impl Field for Option<String> {
	fn read<R: Read + Seek>(reader: &mut R, endian: Endian, base: u64) -> BinResult<Self> {
		offset_string(reader, endian, (base,))
	}
}

impl Field for Vec<f32> {
	fn read<R: Read + Seek>(reader: &mut R, endian: Endian, base: u64) -> BinResult<Self> {
		offset_floats(reader, endian, (base,))
	}
}

impl Field for Vec<Caption> {
	fn read<R: Read + Seek>(reader: &mut R, endian: Endian, base: u64) -> BinResult<Self> {
		at_list(reader, endian, base, 12)
	}
}

impl Field for Vec<[f32; 3]> {
	fn read<R: Read + Seek>(reader: &mut R, endian: Endian, base: u64) -> BinResult<Self> {
		at_list(reader, endian, base, 12)
	}
}

impl Field for Option<Filter> {
	fn read<R: Read + Seek>(reader: &mut R, endian: Endian, base: u64) -> BinResult<Self> {
		at_offset(reader, endian, base, 0x14, |reader| {
			Filter::read_options(reader, endian, ())
		})
	}
}

/// Which parts of a character [`C094`] hides.
///
/// Written only where the command uses it, so its absence is a zero offset.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Filter {
	enable: i32,

	/// Character `0x01`, weapon `0x02`, offhand `0x04`, summon `0x08`.
	filter: i32,

	unknown_4: i32,

	unknown_5: i32,

	unknown_6: i32,
}

/// How long a [`C048`] subtitle stands in one language.
///
/// The index is the language itself: `sub_14185B870` reads this list at `12 *
/// EnvironmentManager.GetCutsceneLanguage()`, and plays the line's voice only where the entry it
/// lands on is enabled. The order is `ja`, `en`, `de`, `fr`, `chs`, a slot the client rejects,
/// `ko`, `tc`, after the suffix table `sub_14185AE20` builds a voice path out of. Nothing the game
/// ships fills the rejected slot or `tc`.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Caption {
	/// One where that language states the line, nought where it states nothing.
	enabled: i32,

	/// How long the line stands, in milliseconds. The client counts it down against the frame's
	/// own elapsed seconds (`Client::UI::Agent::AgentTalkSubtitle.Update`).
	duration: i32,

	unknown_3: i32,
}

macro_rules! commands {
	($(
		$(#[doc = $doc:literal])+
		$magic:ident { $($(#[$meta:meta])* $field:ident: $type:ty),* $(,)? }
	)*) => {
		$(
			$(#[doc = $doc])+
			#[derive(Debug, CopyGetters)]
			#[get_copy = "pub"]
			#[allow(missing_docs)]
			pub struct $magic {
				$($(#[$meta])* $field: $type,)*
			}

			impl $magic {
				fn parse<R: Read + Seek>(
					reader: &mut R,
					endian: Endian,
					base: u64,
				) -> BinResult<Self> {
					Ok(Self {
						$($field: <$type as Field>::read(reader, endian, base)?,)*
					})
				}
			}
		)*

		/// What a command does, one variant per magic this crate models.
		///
		/// Most field names are VFXEditor's reading of the format rather than a measurement, and the
		/// majority are `unknown_n` there too. The catalogue is open by construction: the observed
		/// discriminants run `C002`..`C234` with ~170 numeric slots unallocated, so a magic outside
		/// this set is expected rather than exceptional.
		#[allow(missing_docs)]
		#[derive(Debug)]
		pub enum CommandKind {
			$($(#[doc = $doc])+ $magic($magic),)*

			/// A magic no reference implementation specifies, as the bytes past the preamble.
			Unknown {
				magic: [u8; 4],
				body: Vec<u8>,
			},
		}

		fn kind<R: Read + Seek>(
			reader: &mut R,
			endian: Endian,
			magic: &[u8; 4],
			base: u64,
		) -> BinResult<Option<CommandKind>> {
			Ok(Some(match magic {
				$(
					_ if magic.as_slice() == stringify!($magic).as_bytes() => {
						CommandKind::$magic($magic::parse(reader, endian, base)?)
					}
				)*
				_ => return Ok(None),
			}))
		}
	};
}

commands! {
	/// Plays another timeline.
	C002 { duration: i32, unknown_1: i32, unknown_2: i32, #[getset(skip)] path: Option<String> }

	/// The camera a shot runs through, usable only from a `.cutb`.
	///
	/// Where the camera stands comes from the curve set below rather than from this body. The set's
	/// targets carry a [`role`](super::Curve::role) apiece and hang off one another by
	/// [`parent`](super::Curve::parent); the last target of a role is the one the camera reads.
	/// Role 2 stands the eye, role 3 what it looks at, and role 4 a point one unit along its up
	/// vector, so the camera aims from role 2 towards role 3 with role 3 - role 2 as its forward
	/// and role 4 - role 2 as its up. Where no target carries role 4 the up is world up.
	///
	/// Role 1 is the frame the rest hang off, and the three roles a shot can bind live in
	/// [`bindings`](Self::bindings): its first participant at index 0 is role 1's, index 6 is role
	/// 2's and index 11 is role 3's, each followed by a sub-index and a second participant of its
	/// own, which stands in where the first names nothing. Role 1 takes the participant's rotation
	/// as well as its position unless index 4 is one;
	/// roles 2 and 3 take the position alone. A binding names `0xffffffff` where the role stands
	/// in the world instead.
	///
	/// The set's own channels - the ones at `0x30` and up, on target `0xff` - are the camera
	/// itself:
	///
	/// - `0x34` is a focal length in millimetres, which the game turns into a vertical field of
	///   view as `2 * atan(7.0015101 / focal)` against a frame it fixes at sixteen by nine.
	/// - `0x35` is the roll, in degrees, applied the other way round.
	/// - `0x37` and `0x38` are two strengths and `0x39` a distance, which together focus the blur.
	C004 {
		duration: i32,
		unknown_1: i32,
		/// Id of the `TMFC` holding the camera's curves.
		curve_id: i32,
		#[getset(skip)] name: Option<String>,
		near_plane: f32,
		far_plane: f32,
		/// The participants the shot binds its curve set's roles to, as `CTAL` ids or
		/// `0xffffffff`, interleaved with the sub-indices and flags that go with them. See the
		/// type's own doc for which index belongs to which role.
		#[getset(skip)] bindings: [u32; 17],
	}

	/// Fly text settings.
	C006 { enabled: i32, unknown_2: i32, unknown_3: i32 }

	/// Plays an animation, usable only from a `.pap`.
	C009 { duration: i32, unknown_1: i32, #[getset(skip)] motion: Option<String> }

	/// Plays an animation.
	///
	/// From a `.cutb` the start and end are frames of the clip itself at thirty a second, which is
	/// what a cutscene's own frame numbering runs at: a command naming `cbfm_arms` ends at 155,
	/// and that pack's own binding gives the clip 5.1666665 seconds.
	///
	/// The client takes the name to whichever of the acting object's own loaded packs holds it
	/// (`sub_141B087C0` into `sub_1404200E0`), rather than deriving a path from it.
	C010 {
		duration: i32,
		unknown_1: i32,
		/// `0x01` enables the start and end frames.
		flags: i32,
		animation_start: f32,
		animation_end: f32,
		#[getset(skip)] motion: Option<String>,
		unknown_2: i32,
	}

	/// Fly text.
	C011 { enabled: i32, unknown_2: i32 }

	/// Plays a visual effect.
	C012 {
		duration: i32,
		unknown_1: i32,
		#[getset(skip)] path: Option<String>,
		bind_origin_1: u8,
		bind_type_1: u8,
		bind_id_1: i16,
		bind_origin_2: u8,
		bind_type_2: u8,
		bind_id_2: i16,
		#[getset(skip)] scale: Vec<f32>,
		#[getset(skip)] rotation: Vec<f32>,
		#[getset(skip)] position: Vec<f32>,
		#[getset(skip)] rgba: Vec<f32>,
		visibility: i32,
		unknown_3: i32,
	}

	/// Model animation, driven by an f-curve.
	C013 { duration: i32, unknown_2: i32, curve_id: i32, placement: i32 }

	/// Weapon position.
	C014 { enabled: i32, unknown_2: i32, object_position: i32, object_control: i32 }

	/// Weapon size.
	C015 { duration: i32, unknown_2: i32, weapon_size: i32, object_control: i32 }

	/// A transform to hold a node at, in the space the scene places that node in.
	C018 {
		duration: i32,
		unknown_1: i32,
		translation: [f32; 3],
		/// In radians, as an instance's own rotation is.
		rotation: [f32; 3],
		/// One in every file the game ships.
		scale: [f32; 3],
	}

	/// Whether the node its track names is drawn. A scene runs it against every kind of node it
	/// places, and a cutscene swaps a stand-in for an actor with a pair of them.
	///
	/// `sub_14186BC30` reads the low byte of [`visibility`](Self::visibility) as the state to put
	/// the node in and the byte above it as whether the two objects hanging off that node - a
	/// character's weapons - follow it. The clip's own revert puts the node back to drawn, so a
	/// node no command has reached is drawn.
	C019 {
		duration: i32,
		unknown_1: i32,
		/// Drawn in its low byte, and whether the node's own two sub-objects follow in the byte
		/// above it.
		visibility: i32,
	}

	/// Not named by any reference implementation.
	C021 { unknown_1: i32, unknown_2: i32, unknown_3: i32, unknown_4: i32 }

	/// Summon animation.
	C031 { duration: i32, unknown_1: i32, animation: u16, target_type: i16 }

	/// Crafting delay.
	C033 { enabled: i32, unknown_2: i32 }

	/// Gathering delay.
	C034 { enabled: i32, unknown_2: i32 }

	/// A motion to play by the name a pack the timeline's own loader holds gives it - the same
	/// kind of name [`C010`] plays, not a file path.
	///
	/// The client builds this and [`C090`] out of one class, `sub_141804390`, told apart by the id
	/// it is registered under: 32 here and 75 there, which `sub_14183DE30` reads to pick which of
	/// two runs of the shared object it drives.
	///
	/// A longer form of the command writes seven further words past these, which this does not
	/// read: they hold `1, 0, 1.0, 0, 0, 0, 0` in every file the game ships, and both forms sit in
	/// the same file.
	C040 {
		/// One in every file the game ships.
		enabled: i32,
		unknown_1: i32,
		#[getset(skip)] motion: Option<String>,
		/// A frame count in its low bits, in the same fives [`C090`] states one in, with flags
		/// above it that only the longer form sets.
		unknown_2: i32,
		unknown_3: i32,
		unknown_4: i32,
	}

	/// Footstep.
	C042 { enabled: i32, unknown_2: i32, bind_id: i32, sound_id: i32 }

	/// Summons a weapon.
	C043 {
		duration: i32,
		unknown_1: i32,
		unknown_2: i32,
		weapon_id: i16,
		body_id: i16,
		variant_id: i32,
	}

	/// A subtitle: the row it stands, and how long it stands for in each language.
	///
	/// The row is named rather than numbered. [`key`](Self::key) is the key column of the sheet the
	/// cutscene's own `CTIS` node names - `cut_scene/070/VoiceMan_07003` or
	/// `quest/000/ManFst000_00083`, two string columns of key and text - and the row is the one
	/// whose key matches it exactly. Past the six-figure line number the key carries the speaker's
	/// name, which the client's own parsers stop short of reading
	/// (`TEXT_%6s%3d_%5d_%1x%2d%3d`).
	///
	/// The client shows it through `Client::UI::UIModule.ShowTalkSubtitle`, which lays
	/// `ui/uld/TalkSubtitle.uld` out against a 1280 by 720 frame and clamps its width to sixteen
	/// by nine. `sub_14185B870`, registered under command type 38, is what runs it.
	C048 {
		enabled: i32,
		unknown_1: i32,
		/// Nine in the opening movies and one in the cutscenes the engine draws.
		subtitle_type: i32,
		unknown_3: i32,
		unknown_4: i32,
		unknown_5: i32,
		#[getset(skip)] captions: Vec<Caption>,
		#[getset(skip)] key: Option<String>,
		unknown_6: i32,
		unknown_7: i32,
		unknown_8: i32,
		unknown_9: i32,
	}

	/// Plays a visual effect on the actor whose track runs it.
	///
	/// Built out of the same class as [`C012`], `sub_141851D70`, told apart by the id it is
	/// registered under: 9 there and 39 here, which `sub_141852280` reads to pick which of two
	/// readings of the record to run. This one is `sub_141B67BB0`, and it takes the effect's first
	/// object from the actor the track names rather than from any field here, so nothing in the
	/// body says whom it plays on.
	///
	/// The [`curve_id`](Self::curve_id) set holds five channels on one target, `0x0a` through
	/// `0x0e`, which the clip reads as the effect's colour and alpha.
	C049 {
		enabled: i32,
		unknown_1: i32,
		/// Id of the `TMFC` holding the effect's own curves.
		curve_id: i32,
		#[getset(skip)] path: Option<String>,
		/// A second object to bind to, by the id the scheduler knows it under, or `0xffffffff`
		/// where the effect binds the actor alone.
		second_object: u32,
		unknown_2: u8,
		bind_type_1: u8,
		bind_type_2: u8,
		unknown_3: u8,
		/// Where on each object the effect hangs, or `0xffff` where it stands at the object itself.
		bind_id_1: u16,
		bind_id_2: u16,
		unknown_4: u16,
		/// Read for its lowest bit alone.
		flags: u8,
		unknown_5: u8,
		unknown_6: i32,
		unknown_7: i32,
	}

	/// Voiceline, by sound id.
	C053 {
		unknown_1: i32,
		unknown_2: i32,
		bind_id: i16,
		sound_id: i16,
		unknown_3: i16,
		/// Stop on movement `0x01`, use the bind position `0x02`.
		flags: i16,
	}

	/// Not named by any reference implementation. A scene runs it against a collision box.
	C055 { duration: i32, unknown_1: i32, enabled: i32, unknown_3: i32 }

	/// Not named by any reference implementation. A scene runs it against a sound.
	C056 { duration: i32, unknown_1: i32, unknown_2: f32 }

	/// Not named by any reference implementation. A scene runs it against a sound.
	C057 { duration: i32, unknown_1: i32, unknown_2: f32 }

	/// Not named by any reference implementation. A scene runs it against a visual effect.
	C058 { duration: i32, unknown_1: i32, unknown_2: f32, unknown_3: i32 }

	/// Not named by any reference implementation. A scene runs it against a visual effect.
	C059 { duration: i32, unknown_1: i32, unknown_2: i32 }

	/// Plays a sound.
	C063 {
		loop_duration: i32,
		unknown_1: i32,
		#[getset(skip)] path: Option<String>,
		sound_index: i32,
		/// Use the min/max range `0x01`, stop on animation end `0x02`, use the bind id `0x04`.
		position_flags: u8,
		bind_id: u8,
		unknown_2: i16,
	}

	/// Flinch.
	C067 { enabled: i32, unknown_2: i32 }

	/// Shade colour.
	C068 {
		duration: i32,
		unknown_2: i32,
		#[getset(skip)] color_1: Vec<f32>,
		#[getset(skip)] color_2: Vec<f32>,
	}

	/// Terrain visual effect.
	C075 {
		enabled: i32,
		unknown_1: i32,
		shape: i32,
		#[getset(skip)] scale: Vec<f32>,
		#[getset(skip)] rotation: Vec<f32>,
		#[getset(skip)] position: Vec<f32>,
		#[getset(skip)] rgba: Vec<f32>,
		unknown_3: i32,
		unknown_4: i32,
	}

	/// Not named by any reference implementation. A scene runs it against a shared group.
	C082 { duration: i32, unknown_1: i32, unknown_2: i32, unknown_3: i32 }

	/// Not named by any reference implementation.
	C083 { unknown_1: i32, unknown_2: i32, unknown_3: i32 }

	/// Not named by any reference implementation.
	C084 { unknown_1: i32, unknown_2: i32, unknown_3: i32 }

	/// Animation blending.
	C088 { duration: i32, unknown_2: i32 }

	/// Not named by any reference implementation.
	C089 { duration: i32, unknown_2: i32, unknown_3: i32 }

	/// The expression to put on a face, by the `cfxf_` name a face pack gives it. Built out of the
	/// same class as [`C040`], under a different id.
	C090 {
		/// One in every file the game ships.
		enabled: i32,
		unknown_1: i32,
		#[getset(skip)] motion: Option<String>,
		/// A frame count, in fives.
		unknown_2: i32,
		/// Nought to three.
		unknown_3: i32,
	}

	/// Colour.
	C093 {
		duration: i32,
		unknown_1: i32,
		#[getset(skip)] color_1: Vec<f32>,
		#[getset(skip)] color_2: Vec<f32>,
		unknown_4: i32,
	}

	/// Invisibility, fading between two visibilities.
	C094 {
		fade_time: i32,
		unknown_1: i32,
		start_visibility: f32,
		end_visibility: f32,
		#[getset(skip)] filter: Option<Filter>,
	}

	/// Not named by any reference implementation.
	C095 {
		unknown_1: i32,
		unknown_2: i32,
		unknown_3: i32,
		unknown_4: f32,
		unknown_5: f32,
	}

	/// Hides a weapon.
	C100 {
		enabled: i32,
		unknown_2: i32,
		visibility: i16,
		unknown_3: i16,
		unknown_4: i32,
		unknown_5: i32,
	}

	/// Not named by any reference implementation. A scene runs it against a model or a shared group.
	C104 {
		duration: i32,
		unknown_1: i32,
		/// Zero or one in each of its three low bytes.
		unknown_2: i32,
	}

	/// Visual effect trigger, by `VFXTrigger` row.
	C107 { enabled: i32, unknown_2: i32, trigger_row: i32, unknown_4: i32 }

	/// Not named by any reference implementation. A scene runs it against a light.
	C109 {
		duration: i32,
		unknown_1: i32,
		/// Zero or one in each of its three low bytes.
		unknown_2: i32,
	}

	/// Not named by any reference implementation. A scene runs it against a model.
	C110 {
		duration: i32,
		unknown_1: i32,
		/// Zero or one in each of its two low bytes.
		unknown_2: i32,
	}

	/// A colour a scene gives a light it places, carrying the light's intensity in it.
	C112 { duration: i32, unknown_1: i32, color: [f32; 4] }

	/// A colour a scene gives a model it places.
	C113 { duration: i32, unknown_1: i32, color: [f32; 4] }

	/// Where the world is streamed from, sampled at a fixed interval.
	///
	/// The samples are read straight rather than through a curve set: `sub_141844D50` divides the
	/// elapsed frame by [`step`](Self::step), interpolates between the two samples either side, and
	/// writes the result to the streaming manager's own origin and to the terrain streamer, until
	/// [`until`](Self::until). Nothing sounds or is drawn from it.
	///
	/// [`pass`](Self::pass) indexes a byte of scratch the scheduler clears as a scene opens, which
	/// the command raises as it starts and lowers as it ends. The position is written once that
	/// byte reaches [`passes`](Self::passes), or straight away where the index is `0xffff`, and
	/// never while another command holds the streaming override.
	C114 {
		enabled: i32,
		unknown_1: i32,
		/// Where the origin stands, [`step`](Self::step) frames apart.
		#[getset(skip)] samples: Vec<[f32; 3]>,
		pass: u16,
		passes: u16,
		/// Frames between one sample and the next.
		step: i32,
		/// The frame the track stops at.
		until: f32,
	}

	/// Forced forward movement, driven by an f-curve.
	C117 { duration: i32, unknown_2: i32, curve_id: i32 }

	/// Animation transition.
	C118 { transition_time: i32, unknown_2: i32, unknown_3: i32 }

	/// Controller vibration.
	C120 { duration: i32, unknown_2: i32, wave_type: i32 }

	/// Targetable.
	C124 { enabled: i32, unknown_2: i32, targetable: i32 }

	/// Animation lock.
	C125 { duration: i32, unknown_1: i32 }

	/// Animation cancelled by movement.
	C131 { enabled: i32, unknown_2: i32 }

	/// Not named by any reference implementation.
	C133 { duration: i32, unknown_1: i32, unknown_2: i32, unknown_3: i32 }

	/// Local wind scale.
	C136 { unknown_1: i32, unknown_2: i32 }

	/// Forced movement cancel.
	C139 { enabled: i32, unknown_2: i32 }

	/// Freeze position.
	C142 { duration: i32, unknown_2: i32, position: i32, freeze_location: i32 }

	/// Fishing sound.
	C143 { enabled: i32, unknown_2: i32, bank_id: i32 }

	/// Camera and nameplate control.
	C144 {
		duration: i32,
		unknown_2: i32,
		unknown_3: i32,
		camera: [f32; 2],
		nameplate: [f32; 3],
	}

	/// Blink.
	C161 { enabled: i32, unknown_2: i32, blink: i32, unknown_4: i32 }

	/// Forced camera control, driven by an f-curve.
	C168 {
		duration: i32,
		unknown_2: i32,
		curve_id: i32,
		unknown_4: i32,
		unknown_5: i32,
		unknown_6: i32,
		unknown_7: i32,
		unknown_8: i32,
		unknown_9: i32,
		unknown_10: i32,
		unknown_11: i32,
	}

	/// Plays a visual effect without waiting for it.
	C173 {
		loop_wait: i32,
		unknown_2: i32,
		#[getset(skip)] path: Option<String>,
		bind_origin_1: u8,
		bind_type_1: u8,
		bind_id_1: i16,
		visibility: i32,
		limit: i32,
		unknown_5: i32,
		unknown_6: i32,
		unknown_7: i32,
		unknown_8: i32,
		unknown_9: i32,
		unknown_10: i32,
		unknown_11: i32,
		unknown_12: i32,
	}

	/// Object control.
	C174 {
		duration: i32,
		unknown_2: i32,
		object_position: i32,
		object_control: i32,
		final_position: i32,
		position_delay: i32,
		unknown_6: i32,
	}

	/// Object scaling.
	C175 {
		duration: i32,
		unknown_2: i32,
		object_scale: i32,
		object_control: i32,
		final_scale: i32,
		scale_delay: i32,
		unknown_7: i32,
	}

	/// Forced vertical movement, driven by an f-curve.
	C176 {
		duration: i32,
		unknown_2: i32,
		curve_id: i32,
		unknown_4: i32,
		unknown_5: i32,
		unknown_6: i32,
		unknown_7: i32,
	}

	/// Forced backwards movement, driven by an f-curve.
	C177 {
		duration: i32,
		unknown_2: i32,
		curve_id: i32,
		unknown_4: i32,
		unknown_5: i32,
		unknown_6: i32,
	}

	/// Not named by any reference implementation.
	C178 {
		duration: i32,
		unknown_2: i32,
		curve_id: i32,
		unknown_4: i32,
		unknown_5: i32,
		unknown_6: i32,
	}

	/// Removes a part of the model.
	C187 {
		duration: i32,
		unknown_1: i32,
		part: i32,
		unknown_2: i32,
		unknown_3: i32,
	}

	/// Invisibility.
	C188 { unknown_1: i32, unknown_2: i32 }

	/// Not named by any reference implementation.
	C192 {
		unknown_1: i32,
		unknown_2: i32,
		unknown_3: i32,
		unknown_4: i32,
		unknown_5: i32,
		unknown_6: i32,
		unknown_7: i32,
		unknown_8: i32,
		unknown_9: i32,
		unknown_10: i32,
		unknown_11: i32,
	}

	/// Not named by any reference implementation.
	C194 { unknown_1: i32, unknown_2: i32, unknown_3: i32, unknown_4: i32 }

	/// Voiceline, by number.
	C197 {
		fade_time: i32,
		unknown_2: i32,
		voiceline_number: i32,
		bind_point_id: i32,
		speak_type: i32,
		unknown_6: i32,
	}

	/// Lemure.
	C198 {
		duration: i32,
		unknown_1: i32,
		unknown_2: i32,
		unknown_3: i32,
		summon_id: u8,
		atch_state: u8,
		unknown_4: i16,
		model_id: i16,
		body_id: i16,
		variant: i32,
	}

	/// Freezes an object in place.
	C199 {
		enabled: i32,
		unknown_1: i32,
		bind_point_id: i32,
		unknown_2: i32,
		object_control: i32,
	}

	/// Not named by any reference implementation.
	C202 {
		unknown_1: i32,
		unknown_2: i32,
		unknown_3: i32,
		unknown_4: i32,
		unknown_5: i32,
		unknown_6: i32,
	}

	/// Summoned weapon visibility.
	C203 {
		duration: i32,
		unknown_2: i32,
		bind_point_id: i32,
		rotation: i32,
		object_control: i32,
		no_follow: i32,
		scale_enabled: i16,
		unknown_3: i16,
		scale: f32,
	}

	/// Shroud transform, used by both Reaper and Scholar.
	C204 { duration: i32, unknown_2: i32, unknown_3: i32, unknown_4: i32 }

	/// Locks the facing direction.
	C211 { duration: i32, unknown_2: i32, unknown_3: i32, unknown_4: i32 }

	/// Not named by any reference implementation.
	C212 {
		unknown_1: i32,
		unknown_2: i32,
		unknown_3: i32,
		unknown_4: i32,
		unknown_5: f32,
	}

	/// Not named by any reference implementation.
	C215 { duration: i32, unknown_2: i32, unknown_3: i32, unknown_4: i32 }

	/// Subtitles.
	C216 {
		enabled: i32,
		unknown_2: i32,
		subtitle_type: i32,
		text_id: i32,
		speaker_id: i32,
		duration: f32,
		unknown_7: i32,
		unknown_8: i32,
		unknown_9: i32,
	}

	/// Not named by any reference implementation.
	C225 { duration: i32, unknown_1: i32, unknown_2: i32, unknown_3: i32 }

	/// Background music track.
	C230 {
		enabled: i32,
		unknown_2: i32,
		unknown_3: i32,
		bgm_id: i32,
		unknown_5: i32,
		unknown_6: i32,
		unknown_7: i32,
	}

	/// Not named by any reference implementation.
	C234 {
		unknown_1: i32,
		unknown_2: i32,
		unknown_3: i16,
		unknown_4: i16,
		unknown_5: i32,
		unknown_6: i32,
	}
}

macro_rules! path {
	($($magic:ident)*) => {
		$(impl $magic {
			/// The asset the command plays.
			pub fn path(&self) -> Option<&str> {
				self.path.as_deref()
			}
		})*
	};
}

path!(C002 C012 C049 C063 C173);

macro_rules! motion {
	($($magic:ident)*) => {
		$(impl $magic {
			/// The motion the command plays, by the name a pack gives it. Never a file path: no
			/// string any of these carries holds a separator at all.
			pub fn motion(&self) -> Option<&str> {
				self.motion.as_deref()
			}
		})*
	};
}

motion!(C009 C010 C040 C090);

macro_rules! vectors {
	($($magic:ident { $($field:ident)* })*) => {
		$(impl $magic {
			$(
				/// Three or four floats in every file the game ships, matching the vector the
				/// field holds.
				pub fn $field(&self) -> &[f32] {
					&self.$field
				}
			)*
		})*
	};
}

vectors! {
	C012 { scale rotation position rgba }
	C068 { color_1 color_2 }
	C075 { scale rotation position rgba }
	C093 { color_1 color_2 }
}

impl C004 {
	/// What the shot is called.
	pub fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	/// The participants and the fields between them, in the order the file writes them.
	pub fn bindings(&self) -> &[u32; 17] {
		&self.bindings
	}
}

impl C048 {
	/// The key of the row the subtitle stands, in the sheet the cutscene's `CTIS` names.
	pub fn key(&self) -> Option<&str> {
		self.key.as_deref()
	}

	/// How long the line stands in each language, in the client's own language order.
	pub fn captions(&self) -> &[Caption] {
		&self.captions
	}
}

impl C114 {
	/// Where the streaming origin stands, one sample every [`step`](Self::step) frames.
	pub fn samples(&self) -> &[[f32; 3]] {
		&self.samples
	}
}

impl C094 {
	/// Which parts of the character to hide. `None` where the command carries no filter.
	pub fn filter(&self) -> Option<Filter> {
		self.filter
	}
}
