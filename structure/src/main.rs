#[derive(Debug)]

struct Star {
	name: String,	// star type
	stype: String,	// spectral type
	mass: f32,	// solar mass
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

	// associated function, not method: no self in params
	fn new_star(name: &str, stype: &str, mass: f32) -> Star {
		Star {
			name: String::from(name),
			stype: String::from(stype),
			mass: mass,
		}
	}
}

fn main() {
	let sun = Star {
		name: String::from("Sun"),
		stype: String::from("G2"),
		mass: 1.0,
	};

	let sirius = Star::new_star("Sirius", "A1", 2.0);
	let fomalhaut = Star::new_star("Fomalhaut", "A3", 1.92);
	let zeta_puppis = Star::new_star("Zeta Puppis", "O4", 56.1);

	println!("Sirius mass: {} kg", sirius.mass_kg());
	println!("Fomalhaut mass: {} kg", fomalhaut.mass_kg());

	println!("Star data: {:#?}", sun);
	println!("Star data: {:#?}", fomalhaut);
	println!("Star data: {:#?}", zeta_puppis);

	println!("{} is heavier than {}: {}",
		fomalhaut.name, sirius.name, fomalhaut.is_heavier_than(&sirius));
}
