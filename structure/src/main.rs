#[derive(Debug)]
enum SpectralClass {
	O(u8),
	B(u8),
	A(u8),
	F(u8),
	G(u8),
	K(u8),
	M(u8),
}

#[derive(Debug)]
struct Star {
	name: String,		// star type
	class: SpectralClass,	// spectral type
	mass: f32,		// solar mass
}

impl Star {
	// method: no args
	fn mass_kg(&self) -> f32 {
		// solar mass: (1.98855±0.00025)×10^30 kg
		self.mass * 1.98855 * 10.0_f32.powi(30)
	}

	// method: args
	fn is_heavier_than(&self, other: &Star) -> bool {
		self.mass > other.mass
	}

	// method: match enum with data
	fn get_temp(&self) -> u32 {
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
	fn get_color(&self) -> String {
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
	fn is_blue_giant(&self) -> bool {
		if let SpectralClass::O(_) = self.class {
			true
		} else {
			false
		}
	}
}

impl Star {
	// associated function, not method: no self in params
	fn new_star(name: &str, class: SpectralClass, mass: f32) -> Star {
		Star {
			name: String::from(name),
			class: class,
			mass: mass,
		}
	}
}

fn main() {
	let sun = Star {
		name: String::from("Sun"),
		class: SpectralClass::G(2),
		mass: 1.0,
	};

	let zeta_puppis = Star::new_star("Zeta Puppis", SpectralClass::O(4), 56.1);
	let rigel = Star::new_star("Rigel", SpectralClass::B(8), 23.0);
	let sirius = Star::new_star("Sirius", SpectralClass::A(1), 2.0);
	let fomalhaut = Star::new_star("Fomalhaut", SpectralClass::A(3), 1.92);
	let adhafera = Star::new_star("Zeta Leonis (Adhafera)", SpectralClass::F(0), 3.0);
	let alsafi = Star::new_star("Sigma Draconis (Alsafi)", SpectralClass::K(0), 0.85);
	let betelgeuse = Star::new_star("Betelgeuse", SpectralClass::M(1), 11.6);

	println!("Sirius mass: {} kg", sirius.mass_kg());
	println!("Fomalhaut mass: {} kg", fomalhaut.mass_kg());

	println!("Star data: {:#?}", sun);
	println!("Star data: {:#?}", fomalhaut);
	println!("Star data: {:#?}", zeta_puppis);

	println!("{} is heavier than {}: {}",
		fomalhaut.name, sirius.name, fomalhaut.is_heavier_than(&sirius));
	println!("{} is heavier than {}: {}",
		adhafera.name, alsafi.name, adhafera.is_heavier_than(&alsafi));

	println!("{} temperature: {} K", zeta_puppis.name, zeta_puppis.get_temp());
	println!("{} temperature: {} K", betelgeuse.name, betelgeuse.get_temp());
	println!("{} temperature: {} K", sun.name, sun.get_temp());

	println!("{} color: {}", rigel.name, rigel.get_color());
	println!("{} color: {}", betelgeuse.name, betelgeuse.get_color());

	println!("{} is blue giant: {}", betelgeuse.name, betelgeuse.is_blue_giant());
	println!("{} is blue giant: {}", zeta_puppis.name, zeta_puppis.is_blue_giant());
}
