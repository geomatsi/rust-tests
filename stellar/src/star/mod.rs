use std::fmt;
use std::cmp::Ordering;

pub enum SpectralClass {
    O(u8),
    B(u8),
    A(u8),
    F(u8),
    G(u8),
    K(u8),
    M(u8),
}

pub struct Star {
    pub name: String,     // star name
    class: SpectralClass, // spectral type
    mass: f32,            // solar mass
}

pub trait Spectral {
    fn spectral(&self) -> String {
        String::from("undefined")
    }
}

// traits
impl fmt::Display for SpectralClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SpectralClass::O(s) => write!(f, "O({})", s),
            &SpectralClass::B(s) => write!(f, "B({})", s),
            &SpectralClass::A(s) => write!(f, "A({})", s),
            &SpectralClass::F(s) => write!(f, "F({})", s),
            &SpectralClass::G(s) => write!(f, "G({})", s),
            &SpectralClass::K(s) => write!(f, "K({})", s),
            &SpectralClass::M(s) => write!(f, "M({})", s),
        }
    }
}

impl Spectral for Star {
    fn spectral(&self) -> String {
        format!("{}", self.class)
    }
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}K", self.name, self.spectral(), self.get_temp())
    }
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

    pub fn mass_cmp(s1: &Star, s2: &Star) -> Ordering {
        match s1.mass.partial_cmp(&s2.mass) {
            Some(res) => res,
            None => Ordering::Less,
        }
    }

    pub fn class_cmp(s1: &Star, s2: &Star) -> Ordering {
        let v1 = Star::s2n(s1);
        let v2 = Star::s2n(s2);
        v1.cmp(&v2)
    }

    fn s2n(s: &Star) -> u32 {
        match s.class {
            SpectralClass::O(v) => 256 * 6 + u32::from(v),
            SpectralClass::B(v) => 256 * 5 + u32::from(v),
            SpectralClass::A(v) => 256 * 4 + u32::from(v),
            SpectralClass::F(v) => 256 * 3 + u32::from(v),
            SpectralClass::G(v) => 256 * 2 + u32::from(v),
            SpectralClass::K(v) => 256 * 1 + u32::from(v),
            SpectralClass::M(v) => 256 * 0 + u32::from(v),
        }
    }

    pub fn obj_spectral<T: Spectral>(o: &T) -> String {
        format!("{}", o.spectral())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let s1 = Star::new_star("n1", SpectralClass::O(1), 1.0);
        let s2 = Star::new_star("n2", SpectralClass::B(1), 1.0);
        let s3 = Star::new_star("n3", SpectralClass::A(1), 1.0);
        let s4 = Star::new_star("n4", SpectralClass::F(1), 1.0);
        let s5 = Star::new_star("n5", SpectralClass::G(1), 1.0);
        let s6 = Star::new_star("n6", SpectralClass::K(1), 1.0);

        assert_eq!(s1.get_color().to_lowercase(), "blue");
        assert_eq!(s2.get_color().to_lowercase(), "blue");
        assert_eq!(s3.get_color().to_lowercase(), "white");
        assert_eq!(s4.get_color().to_lowercase(), "yellow");
        assert_eq!(s5.get_color().to_lowercase(), "yellow");
        assert_eq!(s6.get_color().to_lowercase(), "red");
    }

    #[test]
    fn test_blue_giant() {
        let s1 = Star::new_star("n1", SpectralClass::O(1), 1.0);
        let s2 = Star::new_star("n2", SpectralClass::B(1), 1.0);

        assert!(s1.is_blue_giant());
        assert!(!s2.is_blue_giant());
    }
}
