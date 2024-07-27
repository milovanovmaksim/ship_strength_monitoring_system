#[cfg(test)]
mod tests {
    use std::{env, rc::Rc, sync::Once};

    use log::info;

    use crate::{
        core::point::Point,
        strength::{
            deadweight::deadweight_intensity::DeadweightIntensity,
            displacement::displacement_intensity::DisplacementIntensity,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::{shipload::Shipload, shiploads::Shiploads},
            ship::{
                ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
                spatium_functions::SpatiumFunctions,
            },
        },
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
    fn displacement_intensity_test() {
        call_once();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.74);
        let shiploads = Rc::new(Shiploads::new(vec![
            Shipload::new(10.0, Point::new(0.0, 0.0, 0.0), 11.75),
            Shipload::new(10.0, Point::new(11.75, 0.0, 0.0), 11.75),
            Shipload::new(10.0, Point::new(23.5, 0.0, 0.0), 11.75),
            Shipload::new(10.0, Point::new(35.25, 0.0, 0.0), 11.75),
        ]));
        let d_i = DisplacementIntensity::new(
            Rc::new(DeadweightIntensity::new(shiploads, ship_dimensions)),
            Rc::new(LightweightIntensity::from_ship_input_data(
                ship_dimensions,
                Lightweight::new(15350.0),
            )),
            ship_dimensions,
        );
        let tested_d_i = SpatiumFunctions::new(vec![
            SpatiumFunction::new(0, -117.5, -105.75, 49.23, 49.23),
            SpatiumFunction::new(1, -105.75, -94.0, 53.64, 53.64),
            SpatiumFunction::new(2, -94.0, -82.25, 58.05, 58.05),
            SpatiumFunction::new(3, -82.25, -70.5, 62.46, 62.46),
            SpatiumFunction::new(4, -70.5, -58.75, 66.87, 66.87),
            SpatiumFunction::new(5, -58.75, -47.0, 71.28, 71.28),
            SpatiumFunction::new(6, -47.0, -35.25, 75.69, 75.69),
            SpatiumFunction::new(7, -35.25, -23.5, 76.42, 76.42),
            SpatiumFunction::new(8, -23.5, -11.75, 76.42, 76.42),
            SpatiumFunction::new(9, -11.75, 0.0, 76.85, 76.85),
            SpatiumFunction::new(10, 0.0, 11.75, 77.28, 77.28),
            SpatiumFunction::new(11, 11.75, 23.5, 77.28, 77.28),
            SpatiumFunction::new(12, 23.5, 35.25, 77.28, 77.28),
            SpatiumFunction::new(13, 35.25, 47.0, 75.92, 75.92),
            SpatiumFunction::new(14, 47.0, 58.75, 69.91, 69.91),
            SpatiumFunction::new(15, 58.75, 70.5, 64.32, 64.32),
            SpatiumFunction::new(16, 70.5, 82.25, 58.74, 58.74),
            SpatiumFunction::new(17, 82.25, 94.0, 53.15, 53.15),
            SpatiumFunction::new(18, 94.0, 105.75, 47.57, 47.57),
            SpatiumFunction::new(19, 105.75, 117.5, 41.98, 41.98),
        ]);
        let d_i = d_i.displacement_intensity().unwrap();
        assert_eq!(tested_d_i, d_i);
    }
}
