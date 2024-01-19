#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use log::debug;

    use crate::{strength::ship::{ship_dimensions::ShipDimensions, load::shipload::Shipload}, core::point::Point};



    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                env_logger::init();
            }
        )
    }

    #[test]
    fn load_start_coordinate_test() {
        let center_gravity = Point::new(25.23, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let load_start_coordinate = 20.125;
        assert_eq!(load_start_coordinate, shipload.load_start_coordinate());

        let center_gravity = Point::new(-25.23, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let load_start_coordinate = -30.335;
        assert_eq!(load_start_coordinate, shipload.load_start_coordinate());
    }

    #[test]
    fn load_end_coordinate_test() {
        let center_gravity = Point::new(25.23, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let load_end_coordinate = 30.335;
        assert_eq!(load_end_coordinate, shipload.load_end_coordinate());

        let center_gravity = Point::new(-25.23, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let load_end_coordinate = -20.125;
        assert_eq!(load_end_coordinate, shipload.load_end_coordinate());
    }

    #[test]
    fn distances_to_frames_test() {
        let center_gravity = Point::new(25.23, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let distances_to_frames = (0.3, 0.6);
        assert_eq!(distances_to_frames, shipload.distances_to_frames(&ship_dimensions));

    }





}