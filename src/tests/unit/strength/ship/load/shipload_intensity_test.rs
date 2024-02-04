#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use crate::{strength::ship::{ship_dimensions::ShipDimensions,
        load::{shipload::Shipload, shipload_intensity::ShiploadIntensity},
        spatium_functions::SpatiumFunctions, spatium_function::SpatiumFunction}, core::point::Point};

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
    fn shipload_intensity_test() {
        call_once();
        let center_gravity = Point::new(25.23, 0.0, 0.0);
        let shipload = Shipload::new(14.23, center_gravity, 10.21);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let shipload_intensity = ShiploadIntensity::new(&shipload);
        let spatium_functions = vec![
            SpatiumFunction::new(
                13,
                18.75,
                25.0,
                0.97,
                0.97,
            ),
            SpatiumFunction::new(
                14,
                25.0,
                31.25,
                0.12,
                0.12,
            ),
            SpatiumFunction::new(
                14,
                25.0,
                31.25,
                1.10,
                1.10,
            ),
            SpatiumFunction::new(
                13,
                18.75,
                25.0,
                0.09,
                0.09,
            ),
        ];
        let test_shipload_intensity = SpatiumFunctions::new(spatium_functions);
        let shipload_intensity = shipload_intensity.spatium_functions(&ship_dimensions);
        assert_eq!(test_shipload_intensity, shipload_intensity);
    }

}