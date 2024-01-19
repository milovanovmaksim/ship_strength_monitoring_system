#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use log::debug;

    use crate::{strength::ship::{lightweight::lightweight::LightweightIntensity, ship_dimensions::ShipDimensions, load::{shipload::Shipload, load_sharing::LoadSharing}}, core::point::Point};



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
    fn number_shared_loads_test() {
        let center_gravity = Point::new(29.29, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(3, shared_loads.len());
    }

    #[test]
    fn shared_loads_test() {
        let center_gravity = Point::new(29.29, 0.0, 0.0);
        let shipload = Shipload::new(4.2, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let load_sharing = LoadSharing::new(&ship_dimensions, &shipload);
        let test_shared_loads = vec![
            Shipload::new(),
            Shipload::new(),
            Shipload::new()
        ];
        let shared_loads = load_sharing.shared_loads();
        assert_eq!(3, shared_loads.len());
    }
}