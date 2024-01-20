#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use crate::{strength::ship::{ship_dimensions::ShipDimensions,
        load::{shipload::Shipload, load_sharing::LoadSharing}}, core::point::Point};



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
        let center_gravity = Point::new(29.29, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let test_shared_loads = vec![
            Shipload::new(0.34, Point::new(24.6, 0.0, 0.0), 0.82),
            Shipload::new(1.29, Point::new(32.82, 0.0, 0.0), 3.14),
            Shipload::new(2.57, Point::new(28.13, 0.0, 0.0), 6.25)
        ];
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(test_shared_loads, shared_loads);
    }
}