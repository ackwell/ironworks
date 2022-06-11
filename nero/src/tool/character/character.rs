use strum::EnumIter;

// todo rename to character

// TODO: Am I able to derive this entirely from excel? CharaMakeType gets most of the way there - but doesn't map to the model IDs we need
#[derive(Clone, Debug)]
pub struct Character {
	pub race: Race,
	pub tribe: Tribe,
	pub gender: Gender,
	kind: Kind,
}

impl Character {
	pub fn id(&self) -> u32 {
		use Gender as G;
		use Race as R;
		use Tribe as T;

		let base = match (&self.race, &self.gender, &self.tribe) {
			(R::Hyur, G::Male, T::First) => 101,
			(R::Hyur, G::Female, T::First) => 201,
			(R::Hyur, G::Male, T::Second) => 301,
			(R::Hyur, G::Female, T::Second) => 401,
			(R::Elezen, G::Male, _) => 501,
			(R::Elezen, G::Female, _) => 601,
			(R::Miqote, G::Male, _) => 701,
			(R::Miqote, G::Female, _) => 801,
			(R::Roegadyn, G::Male, _) => 901,
			(R::Roegadyn, G::Female, _) => 1001,
			(R::Lalafell, G::Male, _) => 1101,
			(R::Lalafell, G::Female, _) => 1201,
			(R::AuRa, G::Male, _) => 1301,
			(R::AuRa, G::Female, _) => 1401,
			(R::Hrothgar, G::Male, _) => 1501,
			(R::Hrothgar, G::Female, _) => 1601,
			(R::Viera, G::Male, _) => 1701,
			(R::Viera, G::Female, _) => 1801,
		};

		// NPCs are xxx4, rather than xxx1
		match self.kind {
			Kind::Pc => base,
			Kind::Npc => base + 3,
		}
	}

	pub fn fallback(&self) -> Option<Self> {
		use Gender as G;
		use Kind as K;
		use Race as R;
		use Tribe as T;

		// Midlander males are the root - they have no fallback.
		if matches!(
			(self.race, self.tribe, self.gender),
			(R::Hyur, T::First, G::Male,)
		) {
			return None;
		}

		// NPCs fall back to their PC counterpart.
		if self.kind == K::Npc {
			return Some(Self {
				kind: K::Pc,
				..self.clone()
			});
		}

		// Hrothgar falls back to Roe.
		if self.race == R::Hrothgar {
			return Some(Self {
				race: R::Roegadyn,
				..self.clone()
			});
		}

		// Midlander and Lala females fall back to their male counterpart.
		if self.gender == G::Female
			&& matches!(
				(self.race, self.tribe),
				(R::Hyur, T::First) | (R::Lalafell, _)
			) {
			return Some(Self {
				gender: G::Male,
				..self.clone()
			});
		}

		// Everything else falls back to Midlander.
		Some(Self {
			race: R::Hyur,
			tribe: T::First,
			..self.clone()
		})
	}
}

impl Default for Character {
	fn default() -> Self {
		Self {
			race: Race::Hyur,
			tribe: Tribe::First,
			gender: Gender::Male,
			kind: Kind::Pc,
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Race {
	Hyur,
	Elezen,
	Miqote,
	Roegadyn,
	Lalafell,
	AuRa,
	Hrothgar,
	Viera,
}

impl Race {
	// TODO: can I use game strings for this?
	pub fn label(&self) -> &'static str {
		match self {
			Self::Hyur => "Hyur",
			Self::Elezen => "Elezen",
			Self::Miqote => "Miqo'te",
			Self::Roegadyn => "Roegadyn",
			Self::Lalafell => "Lalafell",
			Self::AuRa => "Au Ra",
			Self::Hrothgar => "Hrothgar",
			Self::Viera => "Viera",
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Tribe {
	First,
	Second,
}

impl Tribe {
	pub fn label(&self, race: Race) -> &'static str {
		match (race, self) {
			(Race::Hyur, Self::First) => "Midlander",
			(Race::Hyur, Self::Second) => "Highlander",
			(Race::Elezen, Self::First) => "Wildwood",
			(Race::Elezen, Self::Second) => "Duskwight",
			(Race::Miqote, Self::First) => "Seeker of the Sun",
			(Race::Miqote, Self::Second) => "Keeper of the Moon",
			(Race::Roegadyn, Self::First) => "Sea Wolf",
			(Race::Roegadyn, Self::Second) => "Hellsguard",
			(Race::Lalafell, Self::First) => "Plainsfolk",
			(Race::Lalafell, Self::Second) => "Dunesfolk",
			(Race::AuRa, Self::First) => "Raen",
			(Race::AuRa, Self::Second) => "Xaela",
			(Race::Hrothgar, Self::First) => "Helions",
			(Race::Hrothgar, Self::Second) => "The Lost",
			(Race::Viera, Self::First) => "Rava",
			(Race::Viera, Self::Second) => "Veena",
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Gender {
	Male,
	Female,
}

impl Gender {
	pub fn label(&self) -> &'static str {
		match self {
			Self::Male => "Male",
			Self::Female => "Female",
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum Kind {
	Pc,
	Npc,
}
