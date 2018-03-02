extern crate stellar;

use stellar::star::*;

#[test]
fn test_compare_stars() {
    let proxima_centauri = Star::new_star("Proxima Centauri", SpectralClass::M(6), 0.1221);
    let deneb = Star::new_star("Deneb", SpectralClass::A(2), 19.0);
    let sun = Star::get_sun();

    assert!(!proxima_centauri.is_blue_giant());
    assert!(!deneb.is_blue_giant());
    assert!(!sun.is_blue_giant());

    assert!(deneb.is_heavier_than(&sun));
    assert!(sun.is_heavier_than(&proxima_centauri));
    assert!(!sun.is_heavier_than(&deneb));
}
