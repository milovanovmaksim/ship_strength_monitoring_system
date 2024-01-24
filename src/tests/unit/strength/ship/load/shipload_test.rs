#[cfg(test)]
mod tests {
    use std::{sync::Once, env};

    use crate::{strength::ship::{ship_dimensions::ShipDimensions,
        load::{shipload::Shipload, load_spread::LoadSpread}}, core::point::Point};

    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                let _ = env_logger::try_init();
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
        let distances_to_frames = (0.23, 6.02);
        assert_eq!(distances_to_frames, shipload.distances_to_frames(&ship_dimensions));

        let center_gravity = Point::new(-29.29, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let distances_to_frames = (1.96, 4.29);
        assert_eq!(distances_to_frames, shipload.distances_to_frames(&ship_dimensions));

    }

    #[test]
    fn shared_shipload_test() {
        let shipload = Shipload::new(4.2, Point::new(25.23, 0.0, 0.0), 10.21);
        let test_shipload = Shipload::new(1.7523996082272286, Point::new(23.439999999999998, 0.0, 0.0), 4.260000000000002);
        let shared_shipload = shipload.shared_shipload(21.31, 25.57);
        assert_eq!(test_shipload, shared_shipload);
    }

    #[test]
    fn within_many_spatium_test() {
        call_once();
        let test_shipload = Shipload::new(4.2, Point::new(63.0, 0.0, 0.0), 10.21);
        let test_shipload_2 = Shipload::new(10.0, Point::new(-63.0, 0.0, 0.0), 10.21);
        let test_shipload_3 = Shipload::new(10.0, Point::new(1.0, 0.0, 0.0), 14.0);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_spread = test_shipload.spread(&ship_dimensions);
        assert_eq!(LoadSpread::WithinManySpatiums, load_spread);
        assert_eq!(LoadSpread::WithinManySpatiums, test_shipload_2.spread(&ship_dimensions));
        assert_eq!(LoadSpread::WithinManySpatiums, test_shipload_3.spread(&ship_dimensions));
    }

    #[test]
    fn within_one_spatium_test() {
        let test_shipload = Shipload::new(4.2, Point::new(1.0, 0.0, 0.0), 1.0);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_spread = test_shipload.spread(&ship_dimensions);
        assert_eq!(LoadSpread::WithinOneSpatium, load_spread);
    }

    #[test]
    fn outside_right_frame_test() {
        let test_shipload = Shipload::new(4.2, Point::new(70.0, 0.0, 0.0), 1.0);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_spread = test_shipload.spread(&ship_dimensions);
        assert_eq!(LoadSpread::OutsideRightmostFrame, load_spread);
    }

    #[test]
    fn outside_leftmost_frame_test() {
        let test_shipload = Shipload::new(4.2, Point::new(-70.13, 0.0, 0.0), 1.0);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_spread = test_shipload.spread(&ship_dimensions);
        assert_eq!(LoadSpread::OutsideLeftmostFrame, load_spread);
    }
}