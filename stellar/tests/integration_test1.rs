extern crate stellar;

use stellar::star::*;

#[test]
fn test_sun() {
    let sun: Star = Star::get_sun();

    assert_eq!(sun.get_color().to_lowercase(), "yellow");
    assert_eq!(sun.get_temp(), 6000);
    assert!(!sun.is_blue_giant());
    assert!(!sun.is_heavier_than(&sun));
}
