#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use crate::{strength::ship::{ship_dimensions::ShipDimensions,
        load::{shipload::Shipload, load_sharing::LoadSharing, load_spread::LoadSpread}}, core::point::Point};



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
    fn count_shared_loads_test() {
        call_once();
        let center_gravity = Point::new(29.29, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(3, shared_loads.len());
    }

    #[test]
    fn shared_loads_test() {
        call_once();
        let center_gravity = Point::new(0.0, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 12.5);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let test_shared_loads = vec![
            Shipload::new(2.1, Point::new(-3.125, 0.0, 0.0), 6.25),
            Shipload::new(2.1, Point::new(3.125, 0.0, 0.0), 6.25),
        ];
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_loads_test_2() {
        call_once();
        let center_gravity = Point::new(0.0, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 14.5);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        assert_eq!(LoadSpread::WithinManySpatiums, shipload.spread(&ship_dimensions));
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let test_shared_loads = vec![
            Shipload::new(0.2896551724137931, Point::new( -6.75, 0.0, 0.0 ), 1.0),
            Shipload::new(1.8103448275862069, Point::new(-3.125, 0.0, 0.0), 6.25),
            Shipload::new(1.8103448275862069, Point::new(3.125, 0.0, 0.0), 6.25),
            Shipload::new(0.2896551724137931, Point::new(6.75, 0.0, 0.0 ), 1.0 )];
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_loads_test_3() {
        call_once();
        let center_gravity = Point::new(3.125, 0.0, 0.0);
        let shipload = Shipload::new(10.0, center_gravity, 12.5);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        assert_eq!(LoadSpread::WithinManySpatiums, shipload.spread(&ship_dimensions));
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let test_shared_loads = vec![
            Shipload::new(2.5, Point::new(-1.5625, 0.0, 0.0 ), 3.125),
            Shipload::new(5.0, Point::new(3.125, 0.0, 0.0), 6.25),
            Shipload::new(2.5, Point::new(7.8125, 0.0, 0.0), 3.125),];
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_loads_test_4() {
        call_once();
        let center_gravity = Point::new(-64.0, 0.0, 0.0);
        let shipload = Shipload::new(10.0, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        assert_eq!(LoadSpread::WithinManySpatiums, shipload.spread(&ship_dimensions));
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let test_shared_loads = vec![
            Shipload::new(6.469147894221355, Point::new(-65.80250000000001,0.0, 0.0 ), 6.605000000000004),
            Shipload::new(3.530852105778652, Point::new(-60.6975, 0.0, 0.0), 3.605000000000004)];
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(test_shared_loads, shared_loads);
    }

    #[test]
    fn shared_loads_test_5() {
        call_once();
        let center_gravity = Point::new(64.0, 0.0, 0.0);
        let shipload = Shipload::new(10.0, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        assert_eq!(LoadSpread::WithinManySpatiums, shipload.spread(&ship_dimensions));
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let test_shared_loads = vec![
            Shipload::new(3.530852105778652, Point::new(60.6975, 0.0, 0.0), 3.605000000000004),
            Shipload::new(6.121449559255632, Point::new(65.625, 0.0, 0.0), 6.25),
            Shipload::new(0.3476983349657237, Point::new(68.92750000000001, 0.0, 0.0), 0.355000000000004)];
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(test_shared_loads, shared_loads);
    }
}