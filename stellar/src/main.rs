//
//
//

extern crate stellar;

use stellar::star::*;

use std::io;

fn main() {
    let mut sky: Vec<Star> = Vec::new();
    let sun: Star = Star::get_sun();

    sky.push(Star::new_star("Zeta Puppis", SpectralClass::O(4), 56.1));
    sky.push(Star::new_star("Rigel", SpectralClass::B(8), 23.0));
    sky.push(Star::new_star("Sirius", SpectralClass::A(1), 2.0));
    sky.push(Star::new_star("Fomalhaut", SpectralClass::A(3), 1.92));
    sky.push(Star::new_star(
        "Zeta Leonis (Adhafera)",
        SpectralClass::F(0),
        3.0,
    ));
    sky.push(Star::new_star(
        "Sigma Draconis (Alsafi)",
        SpectralClass::K(0),
        0.85,
    ));
    sky.push(Star::new_star("Betelgeuse", SpectralClass::M(1), 11.6));

    for s in &sky {
        println!(
            "{}: mass {} kg, temp {} K, color {}",
            s.name,
            s.mass_kg(),
            s.get_temp(),
            s.get_color(),
        );
    }

    for s in &sky {
        println!("Star: {}", s);
    }

    for s in &sky {
        println!(
            "{} is heavier than {}: {}",
            s.name,
            sun.name,
            s.is_heavier_than(&sun)
        );
        println!("{} is blue giant: {}", s.name, s.is_blue_giant());
    }

    println!("Compare by mass:");
    sky.sort_by(Star::mass_cmp);
    for s in &sky {
        println!("{}", s.name);
    }

    println!("Compare by spectral class:");
    sky.sort_by(Star::class_cmp);
    for s in &sky {
        println!("{} {}", s.name, Star::obj_spectral(s));
    }

    cmd_line_loop(&sky).expect("failure");
}

fn cmd_line_loop(sky: &Vec<Star>) -> Result<u32, io::Error> {
    loop {
        let mut name = String::new();

        for s in sky {
            println!("{}", s.name);
        }

        println!("Input start name to get info or 'exit':");
        let _ = match io::stdin().read_line(&mut name) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        if name.trim().eq("exit") {
            break;
        }

        for s in sky {
            if name.trim().eq(&s.name) {
                println!("----> {}", s);
                break;
            }
        }
    }

    return Ok(0);
}
