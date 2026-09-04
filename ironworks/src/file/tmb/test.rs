use std::io::{self, Cursor};

use crate::{error::Error, file::File};

use super::{Channel, CommandKind, Curve, Item, Layout, Timeline};

/// Builds a timeline, laying the items out contiguously and everything they point at into a shared
/// pool after them.
///
/// A body holds a placeholder where it names something in the pool; `patches` says which body
/// position that is and where in the pool it lands, which is only knowable once every item's length
/// is known.
#[derive(Default)]
struct Builder {
	items: Vec<Entry>,
	pool: Vec<u8>,
}

struct Entry {
	magic: &'static [u8; 4],
	body: Vec<u8>,
	patches: Vec<(usize, usize)>,
	size: usize,
}

impl Builder {
	fn pool(&mut self, bytes: &[u8]) -> usize {
		let at = self.pool.len();
		self.pool.extend(bytes);
		at
	}

	fn item(&mut self, magic: &'static [u8; 4], body: Vec<u8>, patches: &[(usize, usize)]) {
		let size = body.len() + 8;
		self.items.push(Entry {
			magic,
			body,
			patches: patches.to_vec(),
			size,
		});
	}

	/// Declares a size larger than the body actually written, padding out the difference.
	fn padded(&mut self, magic: &'static [u8; 4], body: Vec<u8>, slack: usize) {
		let size = body.len() + 8 + slack;
		self.items.push(Entry {
			magic,
			body,
			patches: Vec::new(),
			size,
		});
	}

	fn build(self) -> Vec<u8> {
		let mut at = 12;
		let starts: Vec<usize> = self
			.items
			.iter()
			.map(|item| {
				let start = at;
				at += item.size;
				start
			})
			.collect();
		let pool_start = at;

		let mut bytes = Vec::from(*b"TMLB");
		bytes.extend(
			u32::try_from(pool_start + self.pool.len())
				.unwrap()
				.to_le_bytes(),
		);
		bytes.extend(u32::try_from(self.items.len()).unwrap().to_le_bytes());

		for (index, item) in self.items.iter().enumerate() {
			let base = starts[index] + 8;
			let mut body = item.body.clone();
			for (at, target) in &item.patches {
				let offset =
					i32::try_from(pool_start + target).unwrap() - i32::try_from(base).unwrap();
				body[*at..*at + 4].copy_from_slice(&offset.to_le_bytes());
			}
			bytes.extend(*item.magic);
			bytes.extend(u32::try_from(item.size).unwrap().to_le_bytes());
			bytes.extend(&body);
			// Nothing promises an item spends its whole declared size on fields.
			bytes.resize(starts[index] + item.size, 0xCC);
		}

		bytes.extend(&self.pool);
		bytes
	}
}

fn header(id: i16, duration: i16) -> Vec<u8> {
	[id, 0, duration, 3]
		.iter()
		.flat_map(|field| field.to_le_bytes())
		.collect()
}

/// An offset placeholder and a count, as `TMAL`, `TMAC` and `TMTR` write their id lists.
fn list(count: usize) -> Vec<u8> {
	let mut body = vec![0; 4];
	body.extend(u32::try_from(count).unwrap().to_le_bytes());
	body
}

fn ids(values: &[i16]) -> Vec<u8> {
	values.iter().flat_map(|id| id.to_le_bytes()).collect()
}

fn floats(values: &[f32]) -> Vec<u8> {
	values
		.iter()
		.flat_map(|value| value.to_le_bytes())
		.collect()
}

/// A `TMDH`, a `TMAL` naming one actor, that actor, one track, and one `C063` playing a sound.
fn sound_timeline() -> Vec<u8> {
	let mut build = Builder::default();
	let actors = build.pool(&ids(&[2]));
	let tracks = build.pool(&ids(&[3]));
	let commands = build.pool(&ids(&[4]));
	let path = build.pool(b"sound/replace_me.scd\0");

	build.item(b"TMDH", header(1, 480), &[]);
	build.item(b"TMAL", list(1), &[(0, actors)]);

	let mut actor = Vec::new();
	actor.extend(2i16.to_le_bytes());
	actor.extend(0i16.to_le_bytes());
	actor.extend(100i32.to_le_bytes());
	actor.extend(0i32.to_le_bytes());
	actor.extend(list(1));
	build.item(b"TMAC", actor, &[(12, tracks)]);

	let mut track = Vec::new();
	track.extend(3i16.to_le_bytes());
	track.extend(0i16.to_le_bytes());
	track.extend(list(1));
	track.extend(0i32.to_le_bytes());
	build.item(b"TMTR", track, &[(4, commands)]);

	let mut sound = Vec::new();
	sound.extend(4i16.to_le_bytes());
	sound.extend(7i16.to_le_bytes());
	sound.extend(1i32.to_le_bytes());
	sound.extend(0i32.to_le_bytes());
	sound.extend([0; 4]);
	sound.extend(9i32.to_le_bytes());
	sound.extend([5, 6]);
	sound.extend(0i16.to_le_bytes());
	build.item(b"C063", sound, &[(12, path)]);

	build.build()
}

#[test]
fn empty() {
	assert!(matches!(
		Timeline::read(io::empty()),
		Err(Error::Resource(_))
	));
}

#[test]
fn reads_the_head_actors_tracks_and_commands() {
	let file = Timeline::read(Cursor::new(sound_timeline())).unwrap();
	assert_eq!(file.layout(), Layout::Standard);

	let items = file.items();
	assert_eq!(items.len(), 5);

	let Item::Header(head) = &items[0] else {
		panic!("not a header")
	};
	assert_eq!((head.id(), head.duration()), (1, 480));

	let Item::ActorList(list) = &items[1] else {
		panic!("not an actor list")
	};
	assert_eq!(list.actors(), [2]);

	let Item::Actor(actor) = &items[2] else {
		panic!("not an actor")
	};
	assert_eq!((actor.id(), actor.ability_delay()), (2, 100));
	assert_eq!(actor.tracks(), [3]);

	let Item::Track(track) = &items[3] else {
		panic!("not a track")
	};
	assert_eq!(track.commands(), [4]);
	assert!(track.condition().is_empty());

	let Item::Command(command) = &items[4] else {
		panic!("not a command")
	};
	assert_eq!((command.id(), command.time()), (4, 7));
	let CommandKind::C063(sound) = command.kind() else {
		panic!("not a sound")
	};
	assert_eq!(sound.path(), Some("sound/replace_me.scd"));
	assert_eq!(sound.sound_index(), 9);
	assert_eq!((sound.position_flags(), sound.bind_id()), (5, 6));

	// Every id a list names belongs to an item of the timeline.
	let known: Vec<i16> = items.iter().filter_map(Item::id).collect();
	assert_eq!(known, [1, 2, 3, 4]);
}

/// A subtitle names the row it stands rather than numbering it, and states a length apiece for the
/// languages the game ships one in.
#[test]
fn a_subtitle_names_its_row_and_a_length_a_language() {
	let mut build = Builder::default();
	let key = build.pool(b"TEXT_VOICEMAN_07003_003100_GALUF\0");
	let captions = build.pool(
		&[(1, 2519), (1, 2467), (0, 0)]
			.iter()
			.flat_map(|(enabled, duration)| {
				let mut held = i32::to_le_bytes(*enabled).to_vec();
				held.extend(i32::to_le_bytes(*duration));
				held.extend([0; 4]);
				held
			})
			.collect::<Vec<u8>>(),
	);

	let mut subtitle = Vec::new();
	subtitle.extend(9i16.to_le_bytes());
	subtitle.extend(410i16.to_le_bytes());
	subtitle.extend([1, 0, 1, 0, 0, -1].iter().flat_map(|field: &i32| field.to_le_bytes()));
	subtitle.extend(list(3));
	subtitle.extend([0; 4]);
	subtitle.extend([0; 16]);
	build.item(b"C048", subtitle, &[(28, captions), (36, key)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Command(command) = &file.items()[0] else {
		panic!("not a command")
	};
	assert_eq!((command.id(), command.time()), (9, 410));
	let CommandKind::C048(subtitle) = command.kind() else {
		panic!("not a subtitle")
	};
	assert_eq!(subtitle.key(), Some("TEXT_VOICEMAN_07003_003100_GALUF"));
	assert_eq!(subtitle.subtitle_type(), 1);
	let lengths: Vec<(i32, i32)> = subtitle
		.captions()
		.iter()
		.map(|caption| (caption.enabled(), caption.duration()))
		.collect();
	assert_eq!(lengths, [(1, 2519), (1, 2467), (0, 0)]);
}

/// An item may declare more bytes than the fields modelled here spend, so the next item is found
/// from the declared size rather than from wherever the field reads left the cursor.
#[test]
fn an_oversized_item_does_not_desync_the_rest() {
	let mut build = Builder::default();
	build.padded(b"TMDH", header(1, 60), 16);

	let mut lock = Vec::new();
	lock.extend(2i16.to_le_bytes());
	lock.extend(0i16.to_le_bytes());
	lock.extend(30i32.to_le_bytes());
	lock.extend(0i32.to_le_bytes());
	build.item(b"C125", lock, &[]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let items = file.items();
	assert_eq!(items.len(), 2);
	let Item::Command(command) = &items[1] else {
		panic!("the item after the oversized one was not found")
	};
	assert_eq!(command.id(), 2);
	let CommandKind::C125(lock) = command.kind() else {
		panic!("not an animation lock")
	};
	assert_eq!(lock.duration(), 30);
}

/// A track with no commands writes a count of zero beside an offset landing on the very end of the
/// file, so the bound on an offset has to be inclusive of it.
#[test]
fn an_empty_id_list_may_point_at_the_end_of_the_file() {
	let mut build = Builder::default();
	build.item(b"TMDH", header(1, 60), &[]);

	let mut track = Vec::new();
	track.extend(2i16.to_le_bytes());
	track.extend(0i16.to_le_bytes());
	track.extend(list(0));
	track.extend(0i32.to_le_bytes());
	build.item(b"TMTR", track, &[(4, 0)]);

	let bytes = build.build();
	// The pool is empty, so the patched offset resolves to exactly the file length.
	assert_eq!(bytes.len(), 12 + 16 + 24);

	let file = Timeline::read(Cursor::new(bytes)).unwrap();
	let Item::Track(track) = &file.items()[1] else {
		panic!("not a track")
	};
	assert!(track.commands().is_empty());
}

#[test]
fn an_offset_past_the_end_is_rejected() {
	let mut build = Builder::default();
	build.item(b"TMDH", header(1, 60), &[]);

	let mut track = Vec::new();
	track.extend(2i16.to_le_bytes());
	track.extend(0i16.to_le_bytes());
	track.extend(0x4000_0000i32.to_le_bytes());
	track.extend(1u32.to_le_bytes());
	track.extend(0i32.to_le_bytes());
	build.item(b"TMTR", track, &[]);

	assert!(matches!(
		Timeline::read(Cursor::new(build.build())),
		Err(Error::Resource(_))
	));
}

/// An unmodelled `Cxxx` still carries the id its track names it by, and a magic that is not a
/// command at all keeps its whole body.
#[test]
fn an_unmodelled_magic_keeps_its_magic_and_bytes() {
	let mut build = Builder::default();
	build.item(b"TMDH", header(1, 60), &[]);

	let mut command = Vec::new();
	command.extend(9i16.to_le_bytes());
	command.extend(0i16.to_le_bytes());
	command.extend([1, 2, 3, 4, 5, 6, 7, 8]);
	build.item(b"C777", command, &[]);
	build.item(b"ZZZZ", vec![7; 6], &[]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let items = file.items();

	let Item::Command(command) = &items[1] else {
		panic!("an unmodelled Cxxx is still a command")
	};
	assert_eq!(command.id(), 9);
	let CommandKind::Unknown { magic, body } = command.kind() else {
		panic!("modelled a magic it should not know")
	};
	assert_eq!(magic, b"C777");
	assert_eq!(body, &[1, 2, 3, 4, 5, 6, 7, 8]);

	let Item::Unknown(unknown) = &items[2] else {
		panic!("a magic that is not a command should not be read as one")
	};
	assert_eq!(unknown.magic(), *b"ZZZZ");
	assert_eq!(unknown.body(), [7; 6]);
	assert_eq!(items[2].id(), None);
}

/// The path and vectors a `C012` names all live in the shared pool, and a second command naming the
/// same string reads the same value.
#[test]
fn a_visual_effect_reads_its_path_and_vectors() {
	let mut build = Builder::default();
	let path = build.pool(b"vfx/replace_me.avfx\0");
	let scale = build.pool(&floats(&[1.0, 2.0, 3.0]));
	let rgba = build.pool(&floats(&[0.25, 0.5, 0.75, 1.0]));

	build.item(b"TMDH", header(1, 60), &[]);

	let mut vfx = Vec::new();
	vfx.extend(2i16.to_le_bytes());
	vfx.extend(0i16.to_le_bytes());
	vfx.extend(30i32.to_le_bytes());
	vfx.extend(0i32.to_le_bytes());
	vfx.extend([0; 4]);
	vfx.extend([0, 0, 0xFF, 0xFF, 1, 2, 3, 0]);
	// Scale, rotation, position and colour, each an offset beside a count.
	vfx.extend([0; 4]);
	vfx.extend(3u32.to_le_bytes());
	vfx.extend([0; 16]);
	vfx.extend([0; 4]);
	vfx.extend(4u32.to_le_bytes());
	vfx.extend(1i32.to_le_bytes());
	vfx.extend(0i32.to_le_bytes());
	build.item(b"C012", vfx, &[(12, path), (24, scale), (48, rgba)]);

	let mut timeline = Vec::new();
	timeline.extend(3i16.to_le_bytes());
	timeline.extend(0i16.to_le_bytes());
	timeline.extend([0; 16]);
	build.item(b"C002", timeline, &[(16, path)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Command(command) = &file.items()[1] else {
		panic!("not a command")
	};
	let CommandKind::C012(vfx) = command.kind() else {
		panic!("not a visual effect")
	};
	assert_eq!(vfx.path(), Some("vfx/replace_me.avfx"));
	assert_eq!(vfx.scale(), [1.0, 2.0, 3.0]);
	assert_eq!(vfx.rgba(), [0.25, 0.5, 0.75, 1.0]);
	assert!(vfx.rotation().is_empty());
	assert_eq!(vfx.bind_id_1(), -1);
	assert_eq!((vfx.bind_origin_2(), vfx.bind_type_2()), (1, 2));
	assert_eq!(vfx.visibility(), 1);

	let Item::Command(command) = &file.items()[2] else {
		panic!("not a command")
	};
	let CommandKind::C002(timeline) = command.kind() else {
		panic!("not a nested timeline")
	};
	assert_eq!(timeline.path(), Some("vfx/replace_me.avfx"));
}

/// A cutscene camera, which names its curves by the id of a set beside it.
#[test]
fn a_camera_reads_its_planes_and_the_set_it_drives() {
	let mut build = Builder::default();
	let name = build.pool(b"c01\0");

	build.item(b"TMDH", header(1, 160), &[]);

	let mut camera = Vec::new();
	camera.extend(10i16.to_le_bytes());
	camera.extend(0i16.to_le_bytes());
	camera.extend(160i32.to_le_bytes());
	camera.extend(460761i32.to_le_bytes());
	camera.extend(15i32.to_le_bytes());
	camera.extend([0; 4]);
	camera.extend(0.1f32.to_le_bytes());
	camera.extend(1000.0f32.to_le_bytes());
	camera.extend(0xff00_0006u32.to_le_bytes());
	camera.resize(96, 0xFF);
	build.item(b"C004", camera, &[(16, name)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Command(command) = &file.items()[1] else {
		panic!("not a command")
	};
	let CommandKind::C004(camera) = command.kind() else {
		panic!("not a camera")
	};
	assert_eq!(camera.name(), Some("c01"));
	assert_eq!(camera.duration(), 160);
	assert_eq!(camera.curve_id(), 15);
	assert_eq!((camera.near_plane(), camera.far_plane()), (0.1, 1000.0));
	assert_eq!(camera.bindings()[0], 0xff00_0006);
}

/// A `TMFC` reaches its curves from four bytes past the base every other item uses, so a reader
/// sharing the usual base lands short of the first record.
#[test]
fn curves_are_reached_four_bytes_past_the_usual_base() {
	let mut build = Builder::default();
	let curves = build.pool(&{
		let mut bytes = vec![0xEE; 4];
		for (tag, at) in [(b'@', 32i32), (b'A', 40)] {
			bytes.extend([0; 4]);
			bytes.extend([tag, 0, 0, 0]);
			bytes.extend(at.to_le_bytes());
			bytes.extend(1u32.to_le_bytes());
		}
		for (time, value) in [(0.0f32, 1.5f32), (2.0, -3.0)] {
			bytes.extend([0; 4]);
			bytes.extend(time.to_le_bytes());
			bytes.extend([0; 4]);
			bytes.extend(value.to_le_bytes());
			bytes.extend([0; 8]);
		}
		bytes
	});

	build.item(b"TMDH", header(1, 60), &[]);

	let mut curve = Vec::new();
	curve.extend(2i16.to_le_bytes());
	curve.extend(0i16.to_le_bytes());
	curve.extend([0; 4]);
	curve.extend(2u32.to_le_bytes());
	curve.extend(1i32.to_le_bytes());
	curve.extend(36i32.to_le_bytes());
	curve.extend(0i32.to_le_bytes());
	// The patch resolves against the base every other item uses, so naming the filler here is what
	// puts the first record, four bytes further on, where a `TMFC` expects it.
	build.item(b"TMFC", curve, &[(4, curves)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Curves(found) = &file.items()[1] else {
		panic!("not a curve set")
	};
	assert_eq!((found.id(), found.targets(), found.end()), (2, 1, 36));
	let channels: Vec<Option<Channel>> = found.curves().iter().map(Curve::channel).collect();
	assert_eq!(
		channels,
		[Some(Channel::TranslationX), Some(Channel::TranslationY)]
	);
	assert_eq!(found.curves()[0].keys()[0].value(), 1.5);
	assert_eq!(found.curves()[1].keys()[0].time(), 2.0);
}

/// A cutscene camera drives channels past the transform ones, so a curve outside them has to
/// survive with its tag rather than be dropped.
#[test]
fn a_curve_outside_the_transform_channels_keeps_its_tag() {
	let mut build = Builder::default();
	let curves = build.pool(&{
		let mut bytes = vec![0xEE; 4];
		bytes.extend([0; 4]);
		bytes.extend([0x74, 0, 0, 0]);
		bytes.extend(16i32.to_le_bytes());
		bytes.extend(1u32.to_le_bytes());
		bytes.extend([0; 4]);
		bytes.extend(0.0f32.to_le_bytes());
		bytes.extend([0; 4]);
		bytes.extend(35.0f32.to_le_bytes());
		bytes.extend([0; 8]);
		bytes
	});

	build.item(b"TMDH", header(1, 60), &[]);

	let mut curve = Vec::new();
	curve.extend(2i16.to_le_bytes());
	curve.extend(0i16.to_le_bytes());
	curve.extend([0; 4]);
	curve.extend(1u32.to_le_bytes());
	curve.extend([0; 12]);
	build.item(b"TMFC", curve, &[(4, curves)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Curves(found) = &file.items()[1] else {
		panic!("not a curve set")
	};
	assert_eq!(found.curves().len(), 1);
	assert_eq!(found.curves()[0].tag(), 0x74);
	assert_eq!(found.curves()[0].channel(), None);
	assert_eq!(found.curves()[0].keys()[0].value(), 35.0);
}

/// A curve carries its target's role and parent alongside the channel: the role sits above the
/// channel in the sixteen bits the tag opens, so a role past three reaches into the byte behind it.
#[test]
fn a_curve_names_its_target_s_role_and_parent() {
	let mut build = Builder::default();
	let curves = build.pool(&{
		let mut bytes = vec![0xEE; 4];
		for (tag, high, target, parent) in [(0x80u8, 0u8, 0u8, 0xFFu8), (0x00, 1, 1, 0)] {
			bytes.extend([0; 4]);
			bytes.extend([tag, high, target, parent]);
			bytes.extend([0; 8]);
		}
		bytes
	});

	build.item(b"TMDH", header(1, 60), &[]);

	let mut curve = Vec::new();
	curve.extend(2i16.to_le_bytes());
	curve.extend(0i16.to_le_bytes());
	curve.extend([0; 4]);
	curve.extend(2u32.to_le_bytes());
	curve.extend([0; 12]);
	build.item(b"TMFC", curve, &[(4, curves)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Curves(found) = &file.items()[1] else {
		panic!("not a curve set")
	};
	let held: Vec<(u8, u8, Option<u8>, Option<Channel>)> = found
		.curves()
		.iter()
		.map(|curve| {
			(
				curve.target(),
				curve.role(),
				curve.parent(),
				curve.channel(),
			)
		})
		.collect();
	assert_eq!(
		held,
		[
			(0, 2, None, Some(Channel::TranslationX)),
			(1, 4, Some(0), Some(Channel::TranslationX)),
		]
	);
}

/// A span runs as a cubic Hermite over the slopes its two keys carry, unless the key it leaves
/// says to run straight. The tag names the channel in its low six bits, so a target past the
/// first reaches the same component.
#[test]
fn a_span_reads_as_a_cubic_unless_its_first_key_says_otherwise() {
	let mut build = Builder::default();
	let curves = build.pool(&{
		let mut bytes = vec![0xEE; 4];
		bytes.extend([0; 4]);
		bytes.extend([0x80, 0, 2, 0]);
		bytes.extend(16i32.to_le_bytes());
		bytes.extend(3u32.to_le_bytes());
		for (linear, time, rate, value, into, out) in [
			(0u32, 0.0f32, 0.1f32, 0.0f32, 0.0f32, 2.0f32),
			(1, 10.0, 0.1, 10.0, 1.0, 0.0),
			(0, 20.0, 0.0, 4.0, 0.0, 0.0),
		] {
			bytes.extend(linear.to_le_bytes());
			bytes.extend(time.to_le_bytes());
			bytes.extend(rate.to_le_bytes());
			bytes.extend(value.to_le_bytes());
			bytes.extend(into.to_le_bytes());
			bytes.extend(out.to_le_bytes());
		}
		bytes
	});

	build.item(b"TMDH", header(1, 60), &[]);

	let mut curve = Vec::new();
	curve.extend(2i16.to_le_bytes());
	curve.extend(0i16.to_le_bytes());
	curve.extend([0; 4]);
	curve.extend(1u32.to_le_bytes());
	curve.extend([0; 12]);
	build.item(b"TMFC", curve, &[(4, curves)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Curves(found) = &file.items()[1] else {
		panic!("not a curve set")
	};
	let curve = &found.curves()[0];
	assert_eq!(curve.channel(), Some(Channel::TranslationX));
	assert_eq!(curve.target(), 2);
	assert!(!curve.keys()[0].linear());
	assert!(curve.keys()[1].linear());

	// The cubic basis at a quarter along a ten frame span, over slopes 2.0 out and 1.0 in.
	assert_eq!(curve.at(2.5), Some(3.90625));
	assert_eq!(curve.at(12.5), Some(8.5));
	// Held at both ends, and at the last key itself.
	assert_eq!(curve.at(-5.0), Some(0.0));
	assert_eq!(curve.at(20.0), Some(4.0));
	assert_eq!(curve.at(100.0), Some(4.0));
}

#[test]
fn a_track_condition_is_read_where_it_is_present() {
	let mut build = Builder::default();
	let condition = build.pool(&{
		let mut bytes = Vec::new();
		bytes.extend(8u32.to_le_bytes());
		bytes.extend(2u32.to_le_bytes());
		for (operation, value, float) in [(0x13u32, 0x1000_0005u32, 0.0f32), (0x11, 7, 1.5)] {
			bytes.extend(operation.to_le_bytes());
			bytes.extend(value.to_le_bytes());
			bytes.extend(float.to_le_bytes());
		}
		bytes
	});

	build.item(b"TMDH", header(1, 60), &[]);

	let mut track = Vec::new();
	track.extend(2i16.to_le_bytes());
	track.extend(0i16.to_le_bytes());
	track.extend(list(0));
	track.extend([0; 4]);
	build.item(b"TMTR", track, &[(12, condition)]);

	let file = Timeline::read(Cursor::new(build.build())).unwrap();
	let Item::Track(track) = &file.items()[1] else {
		panic!("not a track")
	};
	let steps = track.condition();
	assert_eq!(steps.len(), 2);
	assert_eq!(
		(steps[0].operation(), steps[0].value()),
		(0x13, 0x1000_0005)
	);
	assert_eq!((steps[1].operation(), steps[1].float()), (0x11, 1.5));
}

/// The wide layout repeats its item count where a standard timeline puts its first magic, and runs
/// its header to 0x20.
#[test]
fn the_wide_layout_is_told_apart_by_content() {
	let mut bytes = Vec::from(*b"TMLB");
	bytes.extend(96u32.to_le_bytes());
	bytes.extend(2u32.to_le_bytes());
	bytes.extend(2u32.to_le_bytes());
	bytes.extend([0; 16]);
	for (magic, id) in [(b"TMDH", 1u32), (b"TMAL", 2)] {
		bytes.extend(*magic);
		bytes.extend(32u32.to_le_bytes());
		bytes.extend([0; 8]);
		bytes.extend(id.to_le_bytes());
		bytes.extend([0; 12]);
	}

	let file = Timeline::read(Cursor::new(bytes)).unwrap();
	assert_eq!(file.layout(), Layout::Wide);
	let items = file.items();
	assert_eq!(items.len(), 2);
	for item in items {
		let Item::Unknown(unknown) = item else {
			panic!("a wide item must not be read against the standard layout")
		};
		assert_eq!(unknown.body().len(), 24);
	}
	assert_eq!(items[0].id(), None);
}

#[test]
fn an_item_declaring_more_than_the_timeline_holds_is_rejected() {
	let mut bytes = sound_timeline();
	bytes[16..20].copy_from_slice(&0x0100_0000u32.to_le_bytes());
	assert!(matches!(
		Timeline::read(Cursor::new(bytes)),
		Err(Error::Resource(_))
	));
}

/// A timeline may sit anywhere in a stream, as it does inside a `.pap` or a `.cutb`.
#[test]
fn reads_from_wherever_the_stream_is_positioned() {
	let mut bytes = vec![0xAB; 40];
	bytes.extend(sound_timeline());
	let mut stream = Cursor::new(bytes);
	stream.set_position(40);

	let file = Timeline::read(stream).unwrap();
	let Item::Command(command) = &file.items()[4] else {
		panic!("not a command")
	};
	let CommandKind::C063(sound) = command.kind() else {
		panic!("not a sound")
	};
	assert_eq!(sound.path(), Some("sound/replace_me.scd"));
}

/// A timeline carrying no items has no magic where the layout is sniffed from.
#[test]
fn a_timeline_with_no_items_is_read() {
	let file = Timeline::read(Cursor::new(Builder::default().build())).unwrap();
	assert_eq!(file.layout(), Layout::Standard);
	assert!(file.items().is_empty());
}

/// A command declaring less than the twelve bytes of its own preamble ends before its body starts,
/// which must be an error rather than a body of negative length.
#[test]
fn a_command_shorter_than_its_preamble_is_rejected() {
	let mut bytes = Vec::from(*b"TMLB");
	bytes.extend(28u32.to_le_bytes());
	bytes.extend(1u32.to_le_bytes());
	bytes.extend(*b"C777");
	bytes.extend(8u32.to_le_bytes());
	// Enough file past the item for the id and time reads themselves to succeed.
	bytes.extend([0; 8]);
	assert!(matches!(
		Timeline::read(Cursor::new(bytes)),
		Err(Error::Resource(_))
	));
}
