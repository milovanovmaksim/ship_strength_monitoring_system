#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use crate::{
        core::point::Point,
        strength::{load::shipload::Shipload, ship::ship_dimensions::ShipDimensions},
    };

    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
            env::set_var("RUST_LOG", "debug"); // off / error / warn / info / debug / trace
                                               // env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_BACKTRACE", "full");
            let _ = env_logger::try_init();
        })
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
        assert_eq!(
            distances_to_frames,
            shipload.distances_to_frames(&ship_dimensions)
        );

        let center_gravity = Point::new(-29.29, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let distances_to_frames = (1.96, 4.29);
        assert_eq!(
            distances_to_frames,
            shipload.distances_to_frames(&ship_dimensions)
        );
    }

    #[test]
    fn count_shared_loads_test() {
        call_once();
        let center_gravity = Point::new(29.29, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let shared_loads = shipload.shared_shiploads(&ship_dimensions);
        assert_eq!(3, shared_loads.len());
    }

    #[test]
    fn shared_shiploads_test() {
        call_once();
        let center_gravity = Point::new(0.0, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 12.5);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let test_shared_loads = vec![
            Shipload::new(2.1, Point::new(-3.13, 0.0, 0.0), 6.25),
            Shipload::new(2.1, Point::new(3.13, 0.0, 0.0), 6.25),
        ];
        let shared_loads = shipload.shared_shiploads(&ship_dimensions);
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_shiploads_test_2() {
        call_once();
        let center_gravity = Point::new(0.0, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 14.5);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let test_shared_loads = vec![
            Shipload::new(0.29, Point::new(-6.75, 0.0, 0.0), 1.0),
            Shipload::new(1.81, Point::new(-3.13, 0.0, 0.0), 6.25),
            Shipload::new(1.81, Point::new(3.13, 0.0, 0.0), 6.25),
            Shipload::new(0.29, Point::new(6.75, 0.0, 0.0), 1.0),
        ];
        let shared_loads = shipload.shared_shiploads(&ship_dimensions);
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_shiploads_test_3() {
        call_once();
        let center_gravity = Point::new(3.125, 0.0, 0.0);
        let shipload = Shipload::new(10.0, center_gravity, 12.5);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let test_shared_loads = vec![
            Shipload::new(2.5, Point::new(-1.56, 0.0, 0.0), 3.13),
            Shipload::new(5.0, Point::new(3.13, 0.0, 0.0), 6.25),
            Shipload::new(2.5, Point::new(7.81, 0.0, 0.0), 3.13),
        ];
        let shared_loads = shipload.shared_shiploads(&ship_dimensions);
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_shiploads_test_4() {
        call_once();
        let center_gravity = Point::new(-64.0, 0.0, 0.0);
        let shipload = Shipload::new(10.0, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let test_shared_loads = vec![
            Shipload::new(6.47, Point::new(-65.80, 0.0, 0.0), 6.61),
            Shipload::new(3.53, Point::new(-60.70, 0.0, 0.0), 3.61),
        ];
        let shared_loads = shipload.shared_shiploads(&ship_dimensions);
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_shiploads_test_5() {
        call_once();
        let center_gravity = Point::new(64.0, 0.0, 0.0);
        let shipload = Shipload::new(10.0, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let test_shared_loads = vec![
            Shipload::new(3.53, Point::new(60.70, 0.0, 0.0), 3.61),
            Shipload::new(6.12, Point::new(65.63, 0.0, 0.0), 6.25),
            Shipload::new(0.35, Point::new(68.93, 0.0, 0.0), 0.36),
        ];
        let shared_loads = shipload.shared_shiploads(&ship_dimensions);
        assert_eq!(test_shared_loads, shared_loads);
    }
    #[test]
    fn shared_shiploads_test_6() {
        call_once();
        let center_gravity = Point::new(3.125, 0.0, 0.0);
        let shipload = Shipload::new(10.0, center_gravity, 6.25);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let test_shared_loads = vec![Shipload::new(10.0, Point::new(3.13, 0.0, 0.0), 6.25)];
        let shared_loads = shipload.shared_shiploads(&ship_dimensions);
        assert_eq!(test_shared_loads, shared_loads);
    }
}
