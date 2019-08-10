fn main() {
    println!("run tests: cargo test --bin test-ord");
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    #[derive(Debug, Clone, Copy)]
    pub enum Events {
        /// Empty event
        None,
        /// Battery low event
        BatteryLow,
        /// Accelerometer Event: motion (X-axis motion, Y-axis motion, Z-axis motion)
        AccEvent(bool, bool, bool),
        /// Button event: (pressed)
        ButtonEvent(bool),
        /// Charger event: (plugged)
        ChargerEvent(bool),
        /// Dock Station event: (detected)
        DockEvent(bool),
        /// Front sensors event: (ll, lc, cc, rc, rr)
        FrontSensor(bool, bool, bool, bool, bool),
        /// Bottom sensors event: (l, c, r)
        BottomSensor(bool, bool, bool),
    }

    impl Default for Events {
        fn default() -> Self {
            Events::None
        }
    }

    impl Events {
        fn prio(self) -> u8 {
            let prio = match self {
                Events::None => 0,
                Events::DockEvent(_) => 10,
                Events::ChargerEvent(_) => 20,
                Events::BatteryLow => 30,
                Events::ButtonEvent(_) => 40,
                Events::AccEvent(_, _, _) => 80,
                Events::FrontSensor(_, _, _, _, _) => 90,
                Events::BottomSensor(_, _, _) => 100,
            };

            prio as u8
        }
    }

    impl Eq for Events {}

    impl PartialEq for Events {
        fn eq(&self, other: &Events) -> bool {
            self.prio() == other.prio()
        }
    }

    impl PartialOrd for Events {
        fn partial_cmp(&self, other: &Events) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Events {
        fn cmp(&self, other: &Events) -> Ordering {
            match self {
                Events::None => match other {
                    Events::None => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
                Events::ChargerEvent(_) => match other {
                    Events::ChargerEvent(_) => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
                Events::DockEvent(_) => match other {
                    Events::DockEvent(_) => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
                Events::BatteryLow => match other {
                    Events::BatteryLow => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
                Events::ButtonEvent(_) => match other {
                    Events::ButtonEvent(_) => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
                Events::AccEvent(_, _, _) => match other {
                    Events::AccEvent(_, _, _) => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
                Events::FrontSensor(_, _, _, _, _) => match other {
                    Events::FrontSensor(_, _, _, _, _) => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
                Events::BottomSensor(_, _, _) => match other {
                    Events::BottomSensor(_, _, _) => Ordering::Equal,
                    _ => self.prio().cmp(&other.prio()),
                },
            }
        }
    }

    #[test]
    fn f_cmp_none() {
        assert_eq!(Events::None, Events::None);
    }

    #[test]
    fn f_cmp_batt() {
        assert_eq!(Events::BatteryLow, Events::BatteryLow);
    }

    #[test]
    fn f_cmp_button() {
        assert_eq!(Events::ButtonEvent(true), Events::ButtonEvent(true));
        assert_eq!(Events::ButtonEvent(false), Events::ButtonEvent(false));
        assert_eq!(Events::ButtonEvent(false), Events::ButtonEvent(true));
    }

    #[test]
    fn f_cmp_charger() {
        assert_eq!(Events::ChargerEvent(true), Events::ChargerEvent(true));
        assert_eq!(Events::ChargerEvent(false), Events::ChargerEvent(false));
        assert_eq!(Events::ChargerEvent(true), Events::ChargerEvent(false));
    }

    #[test]
    fn f_cmp_dock() {
        assert_eq!(Events::DockEvent(true), Events::DockEvent(true));
        assert_eq!(Events::DockEvent(false), Events::DockEvent(false));
        assert_eq!(Events::DockEvent(true), Events::DockEvent(false));
    }

    #[test]
    fn f_cmp_front_sensor() {
        assert_eq!(
            Events::BottomSensor(true, true, true),
            Events::BottomSensor(true, true, true)
        );
        assert_eq!(
            Events::BottomSensor(true, false, true),
            Events::BottomSensor(true, false, true)
        );
        assert_eq!(
            Events::BottomSensor(false, true, false),
            Events::BottomSensor(false, true, false)
        );

        assert_eq!(
            Events::BottomSensor(false, false, false),
            Events::BottomSensor(false, true, false)
        );
        assert_eq!(
            Events::BottomSensor(true, true, false),
            Events::BottomSensor(true, false, false)
        );
    }

    #[test]
    fn f_cmp_prio() {
        assert!(Events::None < Events::BatteryLow);
        assert!(Events::None < Events::ChargerEvent(false));
        assert!(Events::None < Events::ChargerEvent(true));

        assert!(Events::ChargerEvent(true) > Events::DockEvent(true));
        assert!(Events::ChargerEvent(true) > Events::DockEvent(false));

        assert!(Events::BatteryLow < Events::ButtonEvent(false));
        assert!(Events::BatteryLow < Events::ButtonEvent(true));
    }
}
