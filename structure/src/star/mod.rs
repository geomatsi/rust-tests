#[derive(Debug)]
pub enum SpectralClass {
	O(u8),
	B(u8),
	A(u8),
	F(u8),
	G(u8),
	K(u8),
	M(u8),
}

#[derive(Debug)]
pub struct Star {
	pub name: String,	// star name
	class: SpectralClass,	// spectral type
	mass: f32,		// solar mass
}

// methods
impl Star {
	// method: no args
	pub fn mass_kg(&self) -> f32 {
		// solar mass: (1.98855±0.00025)×10^30 kg
		self.mass * 1.98855 * 10.0_f32.powi(30)
	}

	// method: args
	pub fn is_heavier_than(&self, other: &Star) -> bool {
		self.mass > other.mass
	}

	// method: match enum with data
	pub fn get_temp(&self) -> u32 {
		match self.class {
			SpectralClass::O(s) => 30000 + 1000 * u32::from(s),
			SpectralClass::B(s) => 20000 + 1000 * u32::from(s),
			SpectralClass::A(s) => 10000 + 1000 * u32::from(s),
			SpectralClass::F(_) => 7500,
			SpectralClass::G(_) => 6000,
			SpectralClass::K(_) => 5000,
			SpectralClass::M(_) => 4000,
		}
	}

	// method: match with placeholder
	pub fn get_color(&self) -> String {
		match self.class {
			SpectralClass::O(_) => String::from("Blue"),
			SpectralClass::B(_) => String::from("Blue"),
			SpectralClass::A(_) => String::from("White"),
			SpectralClass::F(_) => String::from("Yellow"),
			SpectralClass::G(_) => String::from("Yellow"),
			_ => String::from("Red"),
		}
	}

	// method: 'if let' - concise match for specific option
	pub fn is_blue_giant(&self) -> bool {
		if let SpectralClass::O(_) = self.class {
			true
		} else {
			false
		}
	}
}

// associated functions, not methods: no self in params
impl Star {
	pub fn new_star(name: &str, class: SpectralClass, mass: f32) -> Star {
		Star {
			name: String::from(name),
			class: class,
			mass: mass,
		}
	}

	pub fn get_sun() -> Star {
		Star {
			name: String::from("Sun"),
			class: SpectralClass::G(2),
			mass: 1.0,
		}
	}
}
