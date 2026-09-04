use std::io::Cursor;

use binrw::{BinRead, binread};
use getset::{CopyGetters, Getters};

use crate::error::Result;

use super::{invalid, seek, string};

/// Bytes every instance starts with, before its type-specific payload.
const PREFIX: usize = 0x30;

/// What an instance is, as the discriminant in front of it says.
///
/// The client names all 94 slots; the ones below carrying no payload in [`InstanceData`] are named
/// but unmodelled, and their bytes are kept in [`InstanceData::Unknown`].
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum InstanceKind {
	None = 0,
	BgPart = 1,
	Attribute = 2,
	Light = 3,
	Vfx = 4,
	PositionMarker = 5,
	SharedGroup = 6,
	Sound = 7,
	EventNpc = 8,
	BattleNpc = 9,
	RoutePath = 10,
	Character = 11,
	Aetheryte = 12,
	EnvSpace = 13,
	Gathering = 14,
	HelperObject = 15,
	Treasure = 16,
	Clip = 17,
	ClipCtrlPoint = 18,
	ClipCamera = 19,
	ClipLight = 20,
	ClipReserve00 = 21,
	ClipReserve01 = 22,
	ClipReserve02 = 23,
	ClipReserve03 = 24,
	ClipReserve04 = 25,
	ClipReserve05 = 26,
	ClipReserve06 = 27,
	ClipReserve07 = 28,
	ClipReserve08 = 29,
	ClipReserve09 = 30,
	ClipReserve10 = 31,
	ClipReserve11 = 32,
	ClipReserve12 = 33,
	ClipReserve13 = 34,
	ClipReserve14 = 35,
	CutAssetOnlySelectable = 36,
	Player = 37,
	Monster = 38,
	Weapon = 39,
	PopRange = 40,
	ExitRange = 41,
	Lvb = 42,
	MapRange = 43,
	NaviMeshRange = 44,
	EventObject = 45,
	DemiHuman = 46,
	EnvLocation = 47,
	ControlPoint = 48,
	EventRange = 49,
	RestBonusRange = 50,
	QuestMarker = 51,
	Timeline = 52,
	ObjectBehaviourSet = 53,
	Movie = 54,
	ScenarioExd = 55,
	ScenarioText = 56,
	CollisionBox = 57,
	DoorRange = 58,
	LineVfx = 59,
	SoundEnvSet = 60,
	CutActionTimeline = 61,
	CharaScene = 62,
	CutAction = 63,
	EquipPreset = 64,
	ClientPath = 65,
	ServerPath = 66,
	GimmickRange = 67,
	TargetMarker = 68,
	ChairMarker = 69,
	ClickableRange = 70,
	PrefetchRange = 71,
	FateRange = 72,
	PartyMember = 73,
	KeepRange = 74,
	SphereCastRange = 75,
	IndoorObject = 76,
	OutdoorObject = 77,
	EditGroup = 78,
	StableChocobo = 79,
	Unknown80 = 80,
	Unknown81 = 81,
	Unknown82 = 82,
	Decal = 83,
	Unknown84 = 84,
	Unknown85 = 85,
	ColliderLayer7 = 86,
	ColliderLayer8 = 87,
	ColliderLayer9 = 88,
	ColliderLayer10 = 89,
	CullingBox = 90,
	Unknown91 = 91,
	Unknown92 = 92,
	Unknown93 = 93,
}

/// A position, an orientation in radians, and a scale.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Transform {
	translation: [f32; 3],
	/// Euler angles, applied X then Y then Z.
	rotation: [f32; 3],
	scale: [f32; 3],
}

impl Transform {
	/// A transform stated somewhere other than an instance's own header, as a timeline command
	/// states one.
	pub fn new(translation: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) -> Self {
		Self {
			translation,
			rotation,
			scale,
		}
	}
}

/// A colour with an intensity multiplier, so it can exceed what the four bytes alone express.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Colour {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
	intensity: f32,
}

/// A plain colour with no intensity multiplier, unlike [`Colour`].
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Rgba {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}

/// One thing placed on a [`Layer`](super::Layer).
#[derive(Debug, Getters, CopyGetters)]
pub struct Instance {
	#[get_copy = "pub"]
	kind: InstanceKind,

	/// Unique within the layer group. Other instances refer to one by this.
	#[get_copy = "pub"]
	id: u32,

	#[get = "pub"]
	name: String,

	#[get_copy = "pub"]
	transform: Transform,

	#[get = "pub"]
	data: InstanceData,
}

impl Instance {
	pub(crate) fn parse(bytes: &[u8], at: usize, end: usize) -> Result<Self> {
		let head = bytes
			.get(at..end.max(at))
			.filter(|region| region.len() >= PREFIX)
			.ok_or_else(|| invalid(format!("instance at {at:#x} is past the end of the file")))?;

		let mut cursor = Cursor::new(head);
		let kind = InstanceKind::read(&mut cursor)?;
		let id = u32::read_le(&mut cursor)?;
		let name = i32::read_le(&mut cursor)?;
		let transform = Transform::read(&mut cursor)?;

		Ok(Self {
			kind,
			id,
			name: string(bytes, seek(at, name)?),
			transform,
			data: InstanceData::parse(bytes, at, &head[PREFIX..], kind),
		})
	}
}

/// The payload behind an [`Instance`], read according to its [`InstanceKind`].
///
/// A payload nobody has specified, or one an older file laid out differently, lands in
/// [`Unknown`](Self::Unknown) with its bytes intact, so it costs that payload rather than the file.
#[derive(Debug)]
pub enum InstanceData {
	/// The instance carries no payload.
	None,
	BgPart(BgPart),
	Light(LightSource),
	Vfx(Vfx),
	PositionMarker(PositionMarker),
	SharedGroup(SharedGroup),
	Sound(Sound),
	HelperObject(HelperObject),
	EventNpc(EventNpc),
	Character(Character),
	Aetheryte(Aetheryte),
	EnvSpace(EnvSpace),
	Treasure(Treasure),
	Weapon(Weapon),
	PopRange(PopRange),
	ExitRange(ExitRange),
	MapRange(MapRange),
	EventObject(EventObject),
	EnvLocation(EnvLocation),
	EventRange(TriggerBox),
	QuestMarker(QuestMarker),
	CollisionBox(CollisionBox),
	DoorRange(TriggerBox),
	LineVfx(LineVfx),
	ClientPath(ClientPath),
	TargetMarker(TargetMarker),
	ChairMarker(ChairMarker),
	ClickableRange(TriggerBox),
	PrefetchRange(PrefetchRange),
	FateRange(FateRange),
	Decal(Decal),
	CullingBox(CullingBox),
	/// A payload with no reading yet, as the bytes between the instance's prefix and whatever
	/// follows it.
	///
	/// Nothing in the format states an instance's length, so this runs to the next structure and is
	/// an upper bound rather than an exact size: it takes in any string heap that sits between the
	/// two. For the last instance of a layer that is the layer's whole heap.
	Unknown(Vec<u8>),
}

impl InstanceData {
	fn parse(bytes: &[u8], at: usize, payload: &[u8], kind: InstanceKind) -> Self {
		Self::read(bytes, at, payload, kind).unwrap_or_else(|_| Self::Unknown(payload.to_vec()))
	}

	fn read(bytes: &[u8], at: usize, payload: &[u8], kind: InstanceKind) -> Result<Self> {
		// A payload reaching back into the file for a string measures the offset from the instance,
		// so the whole file is passed alongside the slice.
		let mut cursor = Cursor::new(payload);
		Ok(match kind {
			InstanceKind::None => Self::None,
			InstanceKind::BgPart => Self::BgPart(BgPart::parse(bytes, at, &mut cursor)?),
			InstanceKind::Light => Self::Light(LightSource::parse(bytes, at, &mut cursor)?),
			InstanceKind::Vfx => Self::Vfx(Vfx::parse(bytes, at, &mut cursor)?),
			InstanceKind::PositionMarker => {
				Self::PositionMarker(PositionMarker::read(&mut cursor)?)
			}
			InstanceKind::SharedGroup => {
				Self::SharedGroup(SharedGroup::parse(bytes, at, &mut cursor)?)
			}
			InstanceKind::Sound => Self::Sound(Sound::parse(bytes, at, &mut cursor)?),
			InstanceKind::HelperObject => {
				Self::HelperObject(HelperObject::parse(bytes, at, &mut cursor)?)
			}
			InstanceKind::EventNpc => Self::EventNpc(EventNpc::read(&mut cursor)?),
			InstanceKind::Character => Self::Character(Character::read(&mut cursor)?),
			InstanceKind::Aetheryte => Self::Aetheryte(Aetheryte::read(&mut cursor)?),
			InstanceKind::EnvSpace => Self::EnvSpace(EnvSpace::parse(bytes, at, &mut cursor)?),
			InstanceKind::Treasure => Self::Treasure(Treasure::read(&mut cursor)?),
			InstanceKind::Weapon => Self::Weapon(Weapon::read(&mut cursor)?),
			InstanceKind::PopRange => Self::PopRange(PopRange::parse(bytes, at, &mut cursor)?),
			InstanceKind::ExitRange => Self::ExitRange(ExitRange::read(&mut cursor)?),
			InstanceKind::MapRange => Self::MapRange(MapRange::read(&mut cursor)?),
			InstanceKind::EventObject => Self::EventObject(EventObject::read(&mut cursor)?),
			InstanceKind::EnvLocation => {
				Self::EnvLocation(EnvLocation::parse(bytes, at, &mut cursor)?)
			}
			InstanceKind::EventRange => Self::EventRange(TriggerBox::read(&mut cursor)?),
			InstanceKind::QuestMarker => Self::QuestMarker(QuestMarker::read(&mut cursor)?),
			InstanceKind::CollisionBox => {
				Self::CollisionBox(CollisionBox::parse(bytes, at, &mut cursor)?)
			}
			InstanceKind::DoorRange => Self::DoorRange(TriggerBox::read(&mut cursor)?),
			InstanceKind::LineVfx => Self::LineVfx(LineVfx::read(&mut cursor)?),
			InstanceKind::ClientPath => {
				Self::ClientPath(ClientPath::parse(bytes, at, &mut cursor)?)
			}
			InstanceKind::TargetMarker => Self::TargetMarker(TargetMarker::read(&mut cursor)?),
			InstanceKind::ChairMarker => Self::ChairMarker(ChairMarker::read(&mut cursor)?),
			InstanceKind::ClickableRange => Self::ClickableRange(TriggerBox::read(&mut cursor)?),
			InstanceKind::PrefetchRange => Self::PrefetchRange(PrefetchRange::read(&mut cursor)?),
			InstanceKind::FateRange => Self::FateRange(FateRange::read(&mut cursor)?),
			InstanceKind::Decal => Self::Decal(Decal::parse(bytes, at, &mut cursor)?),
			InstanceKind::CullingBox => Self::CullingBox(CullingBox::read(&mut cursor)?),
			_ => Self::Unknown(payload.to_vec()),
		})
	}
}

/// What a [`HelperObject`] stands in for.
///
/// The client builds an [`Instance`] of the kind named here in the helper's place, out of the
/// fields each variant points at.
#[binread]
#[br(little, repr = u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HelperKind {
	/// Nothing is built.
	None = 0,
	/// An object already in the world, reached by [`HelperObject::object_id`].
	Existing = 1,
	/// The player watching, as an [`InstanceKind::Player`].
	Player = 2,
	/// The party member at [`HelperObject::party_index`], as an [`InstanceKind::PartyMember`].
	PartyMember = 3,
	/// [`HelperObject::base_id`] as an [`InstanceKind::EventNpc`].
	EventNpc = 4,
	/// [`HelperObject::base_id`] as an [`InstanceKind::BattleNpc`].
	BattleNpc = 5,
	/// Nothing is built, as for [`Self::None`].
	Reserved6 = 6,
	/// Nothing is built, as for [`Self::None`].
	Reserved7 = 7,
	/// [`HelperObject::nested`], which carries an [`InstanceKind::BgPart`].
	BgPart = 8,
	/// [`HelperObject::nested`], which carries an [`InstanceKind::SharedGroup`].
	SharedGroup = 9,
	/// [`HelperObject::weapon`], as an [`InstanceKind::Weapon`].
	Weapon = 10,
	/// The player's stabled chocobo, as an [`InstanceKind::StableChocobo`].
	StableChocobo = 11,
	/// The member at [`HelperObject::member_index`] of the other roster, as an
	/// [`InstanceKind::PartyMember`].
	PartyMemberAlt = 12,
	/// The member at [`HelperObject::roster_index`], as an [`InstanceKind::Unknown82`].
	Unknown82 = 13,
	/// An [`InstanceKind::Unknown80`], out of the running event's director.
	Unknown80 = 14,
	/// An [`InstanceKind::Unknown81`], out of the running event's director.
	Unknown81 = 15,
	/// [`HelperObject::nested`]'s own path, as an [`InstanceKind::Unknown85`].
	Unknown85 = 16,
}

/// Bytes a [`Placement`] takes.
const PLACEMENT: usize = 0x38;

/// A transform a helper states apart from the instance's own.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Placement {
	transform: Transform,
	/// Bit zero stands [`Self::transform`] in for the instance's own.
	flags: u32,
	unknown: [u32; 4],
}

/// A stand-in for something a file cannot name until it plays: whoever is watching, one of their
/// party, or an asset carried alongside.
///
/// Only a cutscene's `CTAL` places one.
#[derive(Debug, Getters, CopyGetters)]
pub struct HelperObject {
	#[get_copy = "pub"]
	kind: HelperKind,

	/// The object already in the world that [`HelperKind::Existing`] takes over.
	#[get_copy = "pub"]
	object_id: u32,

	/// A row of `ENpcBase`, or of `BNpcBase` under [`HelperKind::BattleNpc`]. The kinds that copy
	/// a live character fall back to it.
	#[get_copy = "pub"]
	base_id: u32,

	#[get_copy = "pub"]
	party_index: u8,

	#[get_copy = "pub"]
	member_index: u8,

	#[get_copy = "pub"]
	roster_index: u8,

	#[get_copy = "pub"]
	weapon: WeaponModel,

	/// Which height to stand the character at, over whatever its own customise data says: zero
	/// leaves it be, and one through five are the 0, 25, 50, 75 and 100 of the game's own slider.
	#[get_copy = "pub"]
	height: u8,

	/// Take [`Self::base_id`] over the live character [`Self::kind`] names.
	#[get_copy = "pub"]
	forces_base_id: bool,

	/// Put the character into its normal mode once it is set up.
	#[get_copy = "pub"]
	normal_mode: bool,

	#[getset(skip)]
	nested: Option<Box<Instance>>,

	#[get_copy = "pub"]
	placement: Option<Placement>,

	#[get_copy = "pub"]
	unknown_1: bool,

	#[get_copy = "pub"]
	unknown_2: bool,

	#[get_copy = "pub"]
	unknown_3: u8,

	#[get_copy = "pub"]
	unknown_4: f32,

	#[get_copy = "pub"]
	unknown_5: [u8; 4],

	/// Nothing in the client reads these.
	#[get_copy = "pub"]
	unknown_6: [f32; 9],

	#[get_copy = "pub"]
	unknown_7: [u8; 4],

	#[get_copy = "pub"]
	unknown_8: i32,
}

#[binread]
#[br(little)]
struct HelperFields {
	#[br(map = |raw: u8| raw != 0, pad_after = 3)]
	unknown_1: bool,
	kind: HelperKind,
	#[br(map = |raw: u8| raw != 0)]
	unknown_2: bool,
	height: u8,
	#[br(map = |raw: u8| raw != 0)]
	normal_mode: bool,
	party_index: u8,
	object_id: u32,
	base_id: u32,
	#[br(map = |raw: u8| raw != 0)]
	forces_base_id: bool,
	unknown_3: u8,
	weapon: WeaponModel,
	member_index: u8,
	roster_index: u8,
	unknown_4: f32,
	nested: i32,
	unknown_5: [u8; 4],
	unknown_6: [f32; 9],
	unknown_7: [u8; 4],
}

impl HelperObject {
	/// Where the kinds naming one take their asset from.
	pub fn nested(&self) -> Option<&Instance> {
		self.nested.as_deref()
	}

	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let end = at + PREFIX + cursor.get_ref().len();
		let fields = HelperFields::read(cursor)?;
		// The oldest cutscenes end a record ahead of the last two fields, so what reads there is
		// either nothing at all or whatever the node lays out next: an offset with no room for a
		// placement behind it inside the record names none.
		let [unknown_8, offset] = <[i32; 2]>::read_le(cursor).unwrap_or_default();
		let placement = match offset {
			0 => None,
			offset => {
				let start = seek(at, offset)?;
				(start.saturating_add(PLACEMENT) <= end).then_some(start)
			}
		};

		let nested = match fields.nested {
			0 => None,
			offset => {
				let start = seek(at, offset)?;
				let bound = placement.unwrap_or(end);
				Some(Box::new(Instance::parse(bytes, start, bound)?))
			}
		};

		Ok(Self {
			kind: fields.kind,
			object_id: fields.object_id,
			base_id: fields.base_id,
			party_index: fields.party_index,
			member_index: fields.member_index,
			roster_index: fields.roster_index,
			weapon: fields.weapon,
			height: fields.height,
			forces_base_id: fields.forces_base_id,
			normal_mode: fields.normal_mode,
			nested,
			placement: match placement {
				None => None,
				Some(found) => Some(Placement::read(&mut seek_to(bytes, found)?)?),
			},
			unknown_1: fields.unknown_1,
			unknown_2: fields.unknown_2,
			unknown_3: fields.unknown_3,
			unknown_4: fields.unknown_4,
			unknown_5: fields.unknown_5,
			unknown_6: fields.unknown_6,
			unknown_7: fields.unknown_7,
			unknown_8,
		})
	}
}

/// How a background model's collision is built.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ModelCollision {
	None = 0,
	Replace = 1,
	Box = 2,
}

/// Whether a model overrides the shadow setting it would otherwise inherit.
#[binread]
#[br(little, repr = u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShadowMode {
	ForceOff = 0,
	ForceOn = 1,
	Inherit = 2,
}

/// A piece of zone scenery: the commonest instance by a wide margin.
#[derive(Debug, Getters, CopyGetters)]
pub struct BgPart {
	#[get = "pub"]
	asset_path: String,
	#[get = "pub"]
	collision_asset_path: String,
	#[get_copy = "pub"]
	collision: ModelCollision,
	#[get_copy = "pub"]
	collision_material_mask: u64,
	#[get_copy = "pub"]
	collision_material_id: u64,
	#[get_copy = "pub"]
	visible: bool,
	#[get_copy = "pub"]
	world_light_shadow_mode: ShadowMode,
	#[get_copy = "pub"]
	object_light_shadow_mode: ShadowMode,
	/// Distance at which the model stops being drawn.
	#[get_copy = "pub"]
	fade_out_distance: f32,
	#[get_copy = "pub"]
	bounding_sphere_size: f32,
}

#[binread]
#[br(little)]
struct BgPartFields {
	asset_path: i32,
	collision_asset_path: i32,
	collision: ModelCollision,
	collision_material_mask_low: u32,
	collision_material_id_low: u32,
	collision_material_mask_high: u32,
	collision_material_id_high: u32,
	_collision_config: i32,
	#[br(map = |raw: u8| raw != 0)]
	visible: bool,
	world_light_shadow_mode: ShadowMode,
	#[br(pad_after = 1)]
	object_light_shadow_mode: ShadowMode,
	fade_out_distance: f32,
	bounding_sphere_size: f32,
}

fn pair(low: u32, high: u32) -> u64 {
	u64::from(low) | (u64::from(high) << 32)
}

/// A reader at `at`, for the structures a payload points off to rather than inlining.
fn seek_to(bytes: &[u8], at: usize) -> Result<Cursor<&[u8]>> {
	bytes
		.get(at..)
		.map(Cursor::new)
		.ok_or_else(|| invalid(format!("offset {at:#x} is past the end of the file")))
}

/// A count the file states, rejected where the bytes behind it cannot hold that many: collecting a
/// range reserves the whole count before the first read fails.
fn count(declared: i32, stride: usize, rest: usize) -> Result<usize> {
	let count = usize::try_from(declared).map_err(|_| invalid(format!("a count of {declared}")))?;
	match count.checked_mul(stride).is_some_and(|size| size <= rest) {
		true => Ok(count),
		false => Err(invalid(format!("a count of {count} in {rest} bytes"))),
	}
}

impl BgPart {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let fields = BgPartFields::read(cursor)?;
		Ok(Self {
			asset_path: string(bytes, seek(at, fields.asset_path)?),
			collision_asset_path: string(bytes, seek(at, fields.collision_asset_path)?),
			collision: fields.collision,
			collision_material_mask: pair(
				fields.collision_material_mask_low,
				fields.collision_material_mask_high,
			),
			collision_material_id: pair(
				fields.collision_material_id_low,
				fields.collision_material_id_high,
			),
			visible: fields.visible,
			world_light_shadow_mode: fields.world_light_shadow_mode,
			object_light_shadow_mode: fields.object_light_shadow_mode,
			fade_out_distance: fields.fade_out_distance,
			bounding_sphere_size: fields.bounding_sphere_size,
		})
	}
}

/// The shape a light throws.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightKind {
	None = 0,
	World = 1,
	Point = 2,
	Spot = 3,
	Flat = 4,
	Line = 5,
	Specular = 6,
}

/// How a point light spreads.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PointLightKind {
	Sphere = 0,
	Hemisphere = 1,
}

/// A placed light.
#[derive(Debug, Getters, CopyGetters)]
pub struct LightSource {
	#[get_copy = "pub"]
	kind: LightKind,
	#[get_copy = "pub"]
	attenuation: f32,
	#[get_copy = "pub"]
	range: f32,
	#[get_copy = "pub"]
	point_light_kind: PointLightKind,
	#[get_copy = "pub"]
	attenuation_cone_coefficient: f32,
	#[get_copy = "pub"]
	spot_angle: f32,
	/// A texture the light projects, where it has one.
	#[get = "pub"]
	texture_path: String,
	#[get_copy = "pub"]
	colour: Colour,
	#[get_copy = "pub"]
	specular_highlights: bool,
	#[get_copy = "pub"]
	bg_part_shadows: bool,
	#[get_copy = "pub"]
	character_shadows: bool,
}

#[binread]
#[br(little)]
struct LightFields {
	kind: LightKind,
	attenuation: f32,
	range: f32,
	point_light_kind: PointLightKind,
	attenuation_cone_coefficient: f32,
	spot_angle: f32,
	texture_path: i32,
	#[br(pad_after = 4)]
	colour: Colour,
	#[br(map = |raw: u8| raw != 0)]
	specular_highlights: bool,
	#[br(map = |raw: u8| raw != 0)]
	bg_part_shadows: bool,
	#[br(map = |raw: u8| raw != 0)]
	character_shadows: bool,
}

impl LightSource {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let fields = LightFields::read(cursor)?;
		Ok(Self {
			kind: fields.kind,
			attenuation: fields.attenuation,
			range: fields.range,
			point_light_kind: fields.point_light_kind,
			attenuation_cone_coefficient: fields.attenuation_cone_coefficient,
			spot_angle: fields.spot_angle,
			texture_path: string(bytes, seek(at, fields.texture_path)?),
			colour: fields.colour,
			specular_highlights: fields.specular_highlights,
			bg_part_shadows: fields.bg_part_shadows,
			character_shadows: fields.character_shadows,
		})
	}
}

/// A placed visual effect.
#[derive(Debug, Getters, CopyGetters)]
pub struct Vfx {
	#[get = "pub"]
	asset_path: String,
	#[get_copy = "pub"]
	soft_particle_fade_range: f32,
	#[get_copy = "pub"]
	colour: Rgba,
	#[get_copy = "pub"]
	auto_play: bool,
	#[get_copy = "pub"]
	no_far_clip: bool,
	/// Where the effect begins and finishes fading, near and far.
	#[get_copy = "pub"]
	fade_near: [f32; 2],
	#[get_copy = "pub"]
	fade_far: [f32; 2],
	/// A z-fighting bias along the camera ray, applied on top of the placement's own transform.
	#[get_copy = "pub"]
	z_correct: f32,
}

#[binread]
#[br(little)]
struct VfxFields {
	asset_path: i32,
	soft_particle_fade_range: f32,
	#[br(pad_before = 4)]
	colour: Rgba,
	#[br(map = |raw: u8| raw != 0)]
	auto_play: bool,
	#[br(map = |raw: u8| raw != 0, pad_after = 2)]
	no_far_clip: bool,
	fade_near: [f32; 2],
	fade_far: [f32; 2],
	z_correct: f32,
}

impl Vfx {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let fields = VfxFields::read(cursor)?;
		Ok(Self {
			asset_path: string(bytes, seek(at, fields.asset_path)?),
			soft_particle_fade_range: fields.soft_particle_fade_range,
			colour: fields.colour,
			auto_play: fields.auto_play,
			no_far_clip: fields.no_far_clip,
			fade_near: fields.fade_near,
			fade_far: fields.fade_far,
			z_correct: fields.z_correct,
		})
	}
}

/// What a position marker marks.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PositionMarkerKind {
	DebugZonePop = 1,
	DebugJump = 2,
	NaviMesh = 3,
	LowQualityEvent = 4,
}

/// A marker the tools place, carrying a comment in two languages.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct PositionMarker {
	kind: PositionMarkerKind,
	comment_jp_offset: u32,
	comment_en_offset: u32,
}

/// Whether a shared group's door opens on approach or is held open or shut.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DoorState {
	Auto = 1,
	Open = 2,
	Closed = 3,
}

/// Whether a shared group's rotation runs.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RotationState {
	Rounding = 1,
	Stopped = 2,
}

/// The state an animated property of a shared group starts in.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AnimationState {
	Play = 0,
	Stop = 1,
	Replay = 2,
	Reset = 3,
}

/// What drives a shared group along its path.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MovePathMode {
	None = 0,
	SharedGroupAction = 1,
	Timeline = 2,
}

/// Which axes a shared group turns on as it moves.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RotationKind {
	NoRotate = 0,
	AllAxis = 1,
	YAxisOnly = 2,
}

/// How a shared group moves and sways along the path it is bound to.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct MovePath {
	mode: MovePathMode,
	#[br(map = |raw: u8| raw != 0, pad_after = 1)]
	auto_play: bool,
	time: u16,
	#[br(map = |raw: u8| raw != 0)]
	loop_playback: bool,
	#[br(map = |raw: u8| raw != 0, pad_after = 2)]
	reverse: bool,
	rotation: RotationKind,
	accelerate_time: u16,
	decelerate_time: u16,
	vertical_swing_range: [f32; 2],
	horizontal_swing_range: [f32; 2],
	swing_move_speed_range: [f32; 2],
	swing_rotation: [f32; 2],
	swing_rotation_speed_range: [f32; 2],
}

/// An `.sgb` placed in the zone, and the state its members start in.
#[derive(Debug, Getters, CopyGetters)]
pub struct SharedGroup {
	#[get = "pub"]
	asset_path: String,
	#[get_copy = "pub"]
	initial_door_state: DoorState,
	#[get_copy = "pub"]
	initial_rotation_state: RotationState,
	#[get_copy = "pub"]
	initial_transform_state: AnimationState,
	#[get_copy = "pub"]
	initial_colour_state: AnimationState,
	#[get_copy = "pub"]
	random_timeline_auto_play: bool,
	#[get_copy = "pub"]
	random_timeline_loop_playback: bool,
	#[get_copy = "pub"]
	collision_controllable_without_event_object: bool,
	/// The [`ClientPath`] the group runs along, or zero.
	#[get_copy = "pub"]
	bound_client_path_instance_id: u32,
	#[get = "pub"]
	move_path: MovePath,
	/// Per-member overrides of what the `.sgb` itself says, undecoded.
	#[get = "pub"]
	overrides: Vec<u8>,
}

#[binread]
#[br(little)]
struct SharedGroupFields {
	asset_path: i32,
	initial_door_state: DoorState,
	overrides: i32,
	_override_count: i32,
	initial_rotation_state: RotationState,
	#[br(map = |raw: u8| raw != 0)]
	random_timeline_auto_play: bool,
	#[br(map = |raw: u8| raw != 0)]
	random_timeline_loop_playback: bool,
	#[br(map = |raw: u8| raw != 0, pad_after = 1)]
	collision_controllable_without_event_object: bool,
	bound_client_path_instance_id: u32,
	#[br(pad_after = 4)]
	move_path: i32,
	initial_transform_state: AnimationState,
	initial_colour_state: AnimationState,
}

impl SharedGroup {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let fields = SharedGroupFields::read(cursor)?;
		let overrides = seek(at, fields.overrides)?;
		let move_path = seek(at, fields.move_path)?;
		Ok(Self {
			asset_path: string(bytes, seek(at, fields.asset_path)?),
			initial_door_state: fields.initial_door_state,
			initial_rotation_state: fields.initial_rotation_state,
			initial_transform_state: fields.initial_transform_state,
			initial_colour_state: fields.initial_colour_state,
			random_timeline_auto_play: fields.random_timeline_auto_play,
			random_timeline_loop_playback: fields.random_timeline_loop_playback,
			collision_controllable_without_event_object: fields
				.collision_controllable_without_event_object,
			bound_client_path_instance_id: fields.bound_client_path_instance_id,
			move_path: MovePath::read(&mut seek_to(bytes, move_path)?)?,
			overrides: bytes.get(overrides..move_path).unwrap_or_default().to_vec(),
		})
	}
}

/// The shape a sound is emitted over.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SoundEffectKind {
	Point = 3,
	PointDirectional = 4,
	Line = 5,
	PolyLine = 6,
	Surface = 7,
	BoardObstruction = 8,
	BoxObstruction = 9,
	PolyLineObstruction = 11,
	PolygonObstruction = 12,
	LineExtController = 13,
	Polygon = 14,
}

/// The distance range and volume a placed [`Sound`] plays over, read out of the fixed block that
/// follows its geometry's own position list. Two more floats sit at the same fixed offsets in
/// every kind that carries one; a corpus sweep found every placement leaving them at `5.0`, so
/// they are not modelled here.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Attenuation {
	/// Full volume at or inside this distance from the listener.
	inner_radius: f32,
	/// Silent at or beyond this distance.
	outer_radius: f32,
	#[br(pad_before = 12)]
	/// Two independently authored `[0, 1]` volume multipliers. Nothing in the file states which
	/// of the two is a placement's "volume" and which is something else, so both are exposed
	/// rather than one being picked; applying their product is a reasonable default.
	volume_a: f32,
	volume_b: f32,
}

impl Attenuation {
	/// Where the block sits past the end of a kind's own position list, for the kinds a corpus
	/// sweep found one in. The obstruction kinds shape another sound's propagation rather than
	/// naming one of their own and carry no volume or range to read.
	fn offset(kind: SoundEffectKind) -> Option<usize> {
		use SoundEffectKind::{Line, Point, PolyLine, Polygon, Surface};
		match kind {
			Point => Some(0x30),
			Line => Some(0x40),
			Surface => Some(0x60),
			PolyLine => Some(0x120),
			Polygon => Some(0x20),
			_ => None,
		}
	}

	fn parse(binary: &[u8], kind: SoundEffectKind) -> Option<Self> {
		let at = Self::offset(kind)?;
		Self::read(&mut seek_to(binary, at).ok()?).ok()
	}
}

/// A placed sound, as an `.scd` and the volume it plays over.
#[derive(Debug, Getters, CopyGetters)]
pub struct Sound {
	#[get = "pub"]
	asset_path: String,
	#[get_copy = "pub"]
	kind: SoundEffectKind,
	#[get_copy = "pub"]
	auto_play: bool,
	#[get_copy = "pub"]
	no_far_clip: bool,
	#[get_copy = "pub"]
	point_selection: u32,
	/// The range and volume this placement plays over, where its kind states one.
	#[get_copy = "pub"]
	attenuation: Option<Attenuation>,
	/// The geometry the [`kind`](Self::kind) is emitted over, undecoded.
	#[get = "pub"]
	binary: Vec<u8>,
}

#[binread]
#[br(little)]
struct SoundParameters {
	kind: SoundEffectKind,
	#[br(map = |raw: u8| raw != 0)]
	auto_play: bool,
	#[br(map = |raw: u8| raw != 0, pad_after = 2)]
	no_far_clip: bool,
	binary: i32,
	binary_size: i32,
	point_selection: u32,
}

impl Sound {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let parameters = i32::read_le(cursor)?;
		let asset_path = i32::read_le(cursor)?;

		let parameters_at = seek(at, parameters)?;
		let parameters = SoundParameters::read(&mut seek_to(bytes, parameters_at)?)?;
		let binary = seek(parameters_at, parameters.binary)?;
		let binary = bytes
			.get(binary..binary + count(parameters.binary_size, 1, bytes.len())?)
			.unwrap_or_default()
			.to_vec();
		Ok(Self {
			asset_path: string(bytes, seek(at, asset_path)?),
			kind: parameters.kind,
			auto_play: parameters.auto_play,
			no_far_clip: parameters.no_far_clip,
			point_selection: parameters.point_selection,
			attenuation: Attenuation::parse(&binary, parameters.kind),
			binary,
		})
	}
}

/// The base an instance is spawned from, as a row of the sheet its kind names.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct GameObject {
	/// A row of `ENpcBase`, `BNpcBase`, `Aetheryte`, `EObj` or `Treasure`, whichever the
	/// instance's kind names.
	base_id: u32,
}

/// A game object that can be placed as a character.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct Character {
	object: GameObject,
	unknown: [u32; 6],
}

/// An event NPC, which the `Level` sheet also projects.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct EventNpc {
	character: Character,
	unknown: [u32; 3],
}

/// An aetheryte, as a row of `Aetheryte`.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct Aetheryte {
	object: GameObject,
	/// The instance this one is attached to, or zero.
	bound_instance_id: u32,
	unknown: u32,
}

/// The volume an environment override covers.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EnvShape {
	Ellipsoid = 1,
	Cuboid = 2,
	Cylinder = 3,
}

/// A volume that swaps in its own environment and sound settings.
#[derive(Debug, Getters, CopyGetters)]
pub struct EnvSpace {
	/// The `.envb` the space applies.
	#[get = "pub"]
	asset_path: String,
	#[get_copy = "pub"]
	bound_instance_id: u32,
	#[get_copy = "pub"]
	shape: EnvShape,
	#[get_copy = "pub"]
	env_map_shooting_point: bool,
	#[get_copy = "pub"]
	priority: u8,
	#[get_copy = "pub"]
	effective_range: f32,
	#[get_copy = "pub"]
	interpolation_time: i32,
	#[get_copy = "pub"]
	reverb: f32,
	#[get_copy = "pub"]
	filter: f32,
	/// The `.essb` the space applies.
	#[get = "pub"]
	sound_asset_path: String,
}

#[binread]
#[br(little)]
struct EnvSpaceFields {
	asset_path: i32,
	bound_instance_id: u32,
	shape: EnvShape,
	#[br(map = |raw: u8| raw != 0)]
	env_map_shooting_point: bool,
	#[br(pad_after = 2)]
	priority: u8,
	effective_range: f32,
	interpolation_time: i32,
	reverb: f32,
	filter: f32,
	sound_asset_path: i32,
}

impl EnvSpace {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let fields = EnvSpaceFields::read(cursor)?;
		Ok(Self {
			asset_path: string(bytes, seek(at, fields.asset_path)?),
			bound_instance_id: fields.bound_instance_id,
			shape: fields.shape,
			env_map_shooting_point: fields.env_map_shooting_point,
			priority: fields.priority,
			effective_range: fields.effective_range,
			interpolation_time: fields.interpolation_time,
			reverb: fields.reverb,
			filter: fields.filter,
			sound_asset_path: string(bytes, seek(at, fields.sound_asset_path)?),
		})
	}
}

/// A treasure coffer, as a row of `Treasure`.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct Treasure {
	object: GameObject,
}

/// Which weapon a character holds, as the three ids that name its model.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct WeaponModel {
	skeleton_id: u16,
	pattern_id: u16,
	image_change_id: u16,
	staining_id: u16,
}

/// A weapon placed on its own.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct Weapon {
	model: WeaponModel,
	#[br(map = |raw: u8| raw != 0)]
	visible: bool,
}

/// What a pop range spawns.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PopKind {
	Player = 1,
	Npc = 2,
	Content = 3,
}

/// Where a group of characters appears, as offsets from the instance.
#[derive(Debug, Getters, CopyGetters)]
pub struct PopRange {
	#[get_copy = "pub"]
	kind: PopKind,
	#[get_copy = "pub"]
	inner_radius_ratio: f32,
	#[get = "pub"]
	positions: Vec<[f32; 3]>,
}

#[binread]
#[br(little)]
struct PopRangeFields {
	kind: PopKind,
	positions: i32,
	position_count: i32,
	inner_radius_ratio: f32,
}

impl PopRange {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let fields = PopRangeFields::read(cursor)?;
		// The positions are measured from the offset field rather than from the instance.
		let mut positions = seek_to(bytes, seek(at + PREFIX + 4, fields.positions)?)?;
		Ok(Self {
			kind: fields.kind,
			inner_radius_ratio: fields.inner_radius_ratio,
			positions: (0..count(
				fields.position_count,
				size_of::<[f32; 3]>(),
				positions.get_ref().len(),
			)?)
				.map(|_| Ok(<[f32; 3]>::read_le(&mut positions)?))
				.collect::<Result<Vec<_>>>()?,
		})
	}
}

/// Which way an exit range sends the player.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitKind {
	ZoneLine = 1,
	Invisible = 2,
}

/// A walkable transition into another zone.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct ExitRange {
	trigger: TriggerBox,
	kind: ExitKind,
	zone_id: u16,
	/// A row of `TerritoryType`.
	territory_type_id: u16,
	index: i32,
	destination_instance_id: u32,
	return_instance_id: u32,
	/// The heading the player arrives on, in radians.
	player_running_direction: f32,
}

/// The area a place name, map, weather and music apply over.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct MapRange {
	trigger: TriggerBox,
	/// A row of `Map`.
	map: u32,
	/// A row of `PlaceName`, for the wider area.
	place_name_block: u32,
	/// A row of `PlaceName`, for the spot itself.
	place_name_spot: u32,
	/// A row of `WeatherRate`.
	weather: u32,
	/// A row of `BGM`.
	#[br(pad_after = 10)]
	bgm: u32,
	housing_block_id: u8,
	#[br(map = |raw: u8| raw != 0)]
	rest_bonus_effective: bool,
	discovery_id: u8,
	#[br(map = |raw: u8| raw != 0)]
	map_enabled: bool,
	#[br(map = |raw: u8| raw != 0)]
	place_name_enabled: bool,
	#[br(map = |raw: u8| raw != 0)]
	discovery_enabled: bool,
	#[br(map = |raw: u8| raw != 0)]
	bgm_enabled: bool,
	#[br(map = |raw: u8| raw != 0)]
	weather_enabled: bool,
	/// Whether the area is a sanctuary.
	#[br(map = |raw: u8| raw != 0)]
	rest_bonus_enabled: bool,
	#[br(map = |raw: u8| raw != 0)]
	bgm_play_zone_in_only: bool,
	#[br(map = |raw: u8| raw != 0)]
	lift_enabled: bool,
	#[br(map = |raw: u8| raw != 0)]
	housing_enabled: bool,
	#[br(map = |raw: u8| raw != 0, pad_after = 1)]
	log_flying_height_max_err: bool,
	#[br(map = |raw: u8| raw != 0)]
	mounts_and_ornaments_disabled: bool,
	#[br(map = |raw: u8| raw != 0)]
	lalafell_only: bool,
}

/// An interactable object, as a row of `EObj`.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct EventObject {
	object: GameObject,
	bound_instance_id: u32,
	unknown: u8,
}

/// The environment a part of the zone is lit and reflected with.
#[derive(Debug, Getters)]
#[get = "pub"]
pub struct EnvLocation {
	/// The `.amb` holding the spherical harmonic ambient light.
	ambient_light_asset_path: String,
	/// The `.tex` cube map reflections are taken from.
	env_map_asset_path: String,
}

impl EnvLocation {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let ambient_light_asset_path = i32::read_le(cursor)?;
		let env_map_asset_path = i32::read_le(cursor)?;
		Ok(Self {
			ambient_light_asset_path: string(bytes, seek(at, ambient_light_asset_path)?),
			env_map_asset_path: string(bytes, seek(at, env_map_asset_path)?),
		})
	}
}

/// A quest marker, wholly unread.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct QuestMarker {
	unknown: [u32; 2],
}

/// The shape a trigger volume takes.
#[binread]
#[br(little, repr = u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TriggerShape {
	None = 0,
	Box = 1,
	Sphere = 2,
	Cylinder = 3,
	Plane = 4,
	Mesh = 5,
	PlaneTwoSided = 6,
}

/// A volume that reports what enters it. The ranges are built on this.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct TriggerBox {
	shape: TriggerShape,
	priority: i16,
	#[br(map = |raw: u8| raw != 0, pad_after = 5)]
	enabled: bool,
}

/// A trigger volume that also carries collision.
#[derive(Debug, Getters, CopyGetters)]
pub struct CollisionBox {
	#[get_copy = "pub"]
	trigger: TriggerBox,
	#[get_copy = "pub"]
	collision_material_mask: u64,
	#[get_copy = "pub"]
	collision_material_id: u64,
	#[get = "pub"]
	collision_asset_path: String,
}

#[binread]
#[br(little)]
struct CollisionBoxFields {
	trigger: TriggerBox,
	collision_material_mask_low: u32,
	collision_material_id_low: u32,
	collision_material_mask_high: u32,
	collision_material_id_high: u32,
	#[br(pad_after = 3)]
	_layer_mask: u8,
	collision_asset_path: i32,
}

impl CollisionBox {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let fields = CollisionBoxFields::read(cursor)?;
		Ok(Self {
			trigger: fields.trigger,
			collision_material_mask: pair(
				fields.collision_material_mask_low,
				fields.collision_material_mask_high,
			),
			collision_material_id: pair(
				fields.collision_material_id_low,
				fields.collision_material_id_high,
			),
			collision_asset_path: string(bytes, seek(at, fields.collision_asset_path)?),
		})
	}
}

/// Which dotted line a boundary is drawn with.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LineStyle {
	Red = 1,
	Blue = 2,
	RedFar = 3,
}

/// The dotted lines marking a zone boundary.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct LineVfx {
	style: LineStyle,
}

/// One node of a [`ClientPath`].
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, CopyGetters)]
#[get_copy = "pub"]
pub struct PathPoint {
	position: [f32; 3],
	id: u16,
	#[br(map = |raw: u8| raw != 0, pad_after = 1)]
	select: bool,
}

/// Bytes one [`PathPoint`] takes on disk.
const PATH_POINT: usize = 16;

/// A route through the zone, which a [`SharedGroup`] can be bound to.
#[derive(Debug, Getters)]
pub struct ClientPath {
	#[get = "pub"]
	control_points: Vec<PathPoint>,
}

impl ClientPath {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let control_points = i32::read_le(cursor)?;
		let control_point_count = i32::read_le(cursor)?;
		let mut points = seek_to(bytes, seek(at, control_points)?)?;
		Ok(Self {
			control_points: (0..count(control_point_count, PATH_POINT, points.get_ref().len())?)
				.map(|_| Ok(PathPoint::read(&mut points)?))
				.collect::<Result<Vec<_>>>()?,
		})
	}
}

/// What a target marker anchors.
#[binread]
#[br(little, repr = i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TargetMarkerKind {
	Target = 0,
	Nameplate = 1,
	LookAt = 2,
	BodyDynamics = 3,
	Root = 4,
	Unknown5 = 5,
	Unknown6 = 6,
}

/// Where the interface anchors itself on an object.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct TargetMarker {
	nameplate_offset_y: f32,
	kind: TargetMarkerKind,
}

/// Whether a seat is sat on or lain on.
#[binread]
#[br(little, repr = u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChairKind {
	Chair = 0,
	Bed = 1,
}

/// Somewhere a character can sit, and which sides they may take.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct ChairMarker {
	#[br(map = |raw: u8| raw != 0)]
	left: bool,
	#[br(map = |raw: u8| raw != 0)]
	right: bool,
	#[br(map = |raw: u8| raw != 0)]
	back: bool,
	kind: ChairKind,
}

/// A volume that starts loading what the [`ExitRange`] it names leads to.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct PrefetchRange {
	trigger: TriggerBox,
	bound_instance_id: u32,
}

/// The area a FATE plays out over.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct FateRange {
	trigger: TriggerBox,
	/// A row of `FateLayoutLabel`.
	fate_layout_label_id: u32,
}

/// A texture projected onto whatever the instance is placed against.
#[derive(Debug, Getters)]
#[get = "pub"]
pub struct Decal {
	diffuse_path: String,
	normal_path: String,
	specular_path: String,
}

impl Decal {
	fn parse(bytes: &[u8], at: usize, cursor: &mut Cursor<&[u8]>) -> Result<Self> {
		let paths = <[i32; 3]>::read_le(cursor)?;
		let path = |offset| -> Result<String> { Ok(string(bytes, seek(at, offset)?)) };
		Ok(Self {
			diffuse_path: path(paths[0])?,
			normal_path: path(paths[1])?,
			specular_path: path(paths[2])?,
		})
	}
}

/// A volume outside which the scenery it covers is not drawn.
#[binread]
#[br(little)]
#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct CullingBox {
	unknown: u32,
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use super::{Attenuation, SoundEffectKind, Vfx};

	/// A `Point` sound's geometry, offsets measured against real files: the core block starts at
	/// `0x30`, right after the one position it carries.
	fn point_geometry(inner: f32, outer: f32, volume_a: f32, volume_b: f32) -> Vec<u8> {
		let mut bytes = vec![0u8; 0x30];
		bytes.extend(inner.to_le_bytes());
		bytes.extend(outer.to_le_bytes());
		bytes.extend([0u8; 12]);
		bytes.extend(volume_a.to_le_bytes());
		bytes.extend(volume_b.to_le_bytes());
		bytes
	}

	#[test]
	fn reads_a_point_attenuation_block() {
		let geometry = point_geometry(5.0, 70.0, 0.8, 0.65);
		let attenuation = Attenuation::parse(&geometry, SoundEffectKind::Point).unwrap();
		assert_eq!(attenuation.inner_radius(), 5.0);
		assert_eq!(attenuation.outer_radius(), 70.0);
		assert_eq!(attenuation.volume_a(), 0.8);
		assert_eq!(attenuation.volume_b(), 0.65);
	}

	/// An obstruction shapes another sound's propagation rather than naming one of its own, so it
	/// has no attenuation block to read at all.
	#[test]
	fn an_obstruction_has_no_attenuation() {
		let geometry = point_geometry(5.0, 70.0, 0.8, 0.65);
		assert!(Attenuation::parse(&geometry, SoundEffectKind::BoardObstruction).is_none());
	}

	#[test]
	fn truncated_geometry_has_no_attenuation() {
		let geometry = vec![0u8; 0x10];
		assert!(Attenuation::parse(&geometry, SoundEffectKind::Point).is_none());
	}

	/// A `Vfx`'s fields, laid out the way Lumina's `VFXInstanceObject.Read` states them: a 4-byte
	/// pad ahead of a plain (non-HDRI) colour, a 2-byte pad after the two flag bytes, and a
	/// trailing `z_correct` this crate did not model until this fix.
	fn vfx_fields(asset_path: &str) -> Vec<u8> {
		let mut bytes = vec![0u8; 40];
		bytes[0..4].copy_from_slice(&40i32.to_le_bytes());
		bytes[4..8].copy_from_slice(&0.25f32.to_le_bytes());
		bytes[12..16].copy_from_slice(&[10, 20, 30, 40]);
		bytes[16] = 1;
		bytes[17] = 0;
		bytes[20..24].copy_from_slice(&1.0f32.to_le_bytes());
		bytes[24..28].copy_from_slice(&2.0f32.to_le_bytes());
		bytes[28..32].copy_from_slice(&3.0f32.to_le_bytes());
		bytes[32..36].copy_from_slice(&4.0f32.to_le_bytes());
		bytes[36..40].copy_from_slice(&5.0f32.to_le_bytes());
		bytes.extend(asset_path.as_bytes());
		bytes.push(0);
		bytes
	}

	#[test]
	fn reads_a_vfx_past_its_padding() {
		let bytes = vfx_fields("b2923_aet1_o.avfx");
		let mut cursor = Cursor::new(&bytes[..40]);
		let vfx = Vfx::parse(&bytes, 0, &mut cursor).unwrap();
		assert_eq!(vfx.asset_path(), "b2923_aet1_o.avfx");
		assert_eq!(vfx.soft_particle_fade_range(), 0.25);
		assert_eq!(
			(
				vfx.colour().red(),
				vfx.colour().green(),
				vfx.colour().blue(),
				vfx.colour().alpha()
			),
			(10, 20, 30, 40)
		);
		assert!(vfx.auto_play());
		assert!(!vfx.no_far_clip());
		assert_eq!(vfx.fade_near(), [1.0, 2.0]);
		assert_eq!(vfx.fade_far(), [3.0, 4.0]);
		assert_eq!(vfx.z_correct(), 5.0);
	}
}
