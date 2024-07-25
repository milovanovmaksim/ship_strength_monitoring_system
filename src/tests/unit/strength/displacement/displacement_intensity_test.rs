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
        let d_i_v = d_i.displacement_intensity().unwrap();
        // let tested_d_i = SpatiumFunctions::new(vec![SpatiumFunction::new(1.0)]);
        info!("{:?}", d_i_v);
    }
}
