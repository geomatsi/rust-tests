//
//
//

extern crate structure;

use structure::star::*;

fn main() {
	let sun = Star::get_sun();

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
