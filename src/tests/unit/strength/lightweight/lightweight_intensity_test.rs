#[cfg(test)]
mod tests {
    use log::debug;
    use std::{env, sync::Once};

    use crate::strength::{
        lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
        ship::ship_dimensions::ShipDimensions,
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
    fn test_lightweight_intensity() {
        call_once();
        let test_weight = 1546.02;
        let lightweight = Lightweight::new(test_weight);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.5);
        let test_lightweight_intensity =
            LightweightIntensity::from_ship_input_data(ship_dimensions, lightweight);
        let lightweight_intensity = test_lightweight_intensity.lightweight_intensity();
        let mut computed_weight = 0.0;
        for spatium in lightweight_intensity.as_ref() {
            computed_weight += spatium.integral();
        }
        assert!(
            (computed_weight - test_weight).abs() / computed_weight.min(test_weight) * 100.0 < 0.5,
        );
    }
}
