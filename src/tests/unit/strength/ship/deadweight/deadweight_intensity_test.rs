#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use crate::{strength::ship::{ship_dimensions::ShipDimensions,
        load::{shipload::Shipload, shiploads::Shiploads}, deadweight::deadweight_intensity::DeadweightIntensity, spatium_functions::SpatiumFunctions, spatium_function::SpatiumFunction}, core::point::Point};



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
    fn spatium_functions_test() {
        call_once();
        let shiploads = Shiploads::new(vec![
            Shipload::new(4.2, Point::new(25.23, 0.0, 0.0), 10.21),
            Shipload::new(5.0, Point::new(64.0, 0.0, 0.0), 1.0),
            Shipload::new(5.0, Point::new(-64.0, 0.0, 0.0), 1.0)
        ]);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let deadweight_intensity = DeadweightIntensity::new(shiploads, ship_dimensions);
        let test_spatium_functions = SpatiumFunctions::new(
            vec![
                SpatiumFunction::new(
                    0,
                    -62.5,
                    -56.25,
                    1.39,
                    1.39,
                ),
                SpatiumFunction::new(
                    1,
                    -56.25,
                    -50.0,
                    -0.59,
                    -0.59,
                ),
                SpatiumFunction::new(
                    2,
                    -50.0,
                    -43.75,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    3,
                    -43.75,
                    -37.5,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    4,
                    -37.5,
                    -31.25,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    5,
                    -31.25,
                    -25.0,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    6,
                    -25.0,
                    -18.75,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    7,
                    -18.75,
                    -12.5,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    8,
                    -12.5,
                    -6.25,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    9,
                    -6.25,
                    0.0,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    10,
                    0.0,
                    6.25,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    11,
                    6.25,
                    12.5,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    12,
                    12.5,
                    18.75,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    13,
                    18.75,
                    25.0,
                    0.31999999999999995,
                    0.31999999999999995,
                ),
                SpatiumFunction::new(
                    14,
                    25.0,
                    31.25,
                    0.36,
                    0.36,
                ),
                SpatiumFunction::new(
                    15,
                    31.25,
                    37.5,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    16,
                    37.5,
                    43.75,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    17,
                    43.75,
                    50.0,
                    0.0,
                    0.0,
                ),
                SpatiumFunction::new(
                    18,
                    50.0,
                    56.25,
                    -0.59,
                    -0.59,
                ),
                SpatiumFunction::new(
                    19,
                    56.25,
                    62.5,
                    1.39,
                    1.39,
                ),
            ]
        );
        let spatium_functions = deadweight_intensity.deadweight_intensity();
        assert_eq!(test_spatium_functions, spatium_functions);

    }
}