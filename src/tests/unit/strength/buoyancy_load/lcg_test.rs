#[cfg(test)]
mod tests {
    use crate::strength::ship::ship_dimensions::ShipDimensions;
    use crate::{
        core::{point::Point, round::Round},
        strength::{
            buoyancy_intensity::lcg::LCG,
            deadweight::deadweight_intensity::DeadweightIntensity,
            displacement::displacement_intensity::DisplacementIntensity,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::{shipload::Shipload, shiploads::Shiploads},
        },
    };
    use std::{env, sync::Once};

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
    fn lcg_ok_test() {
        call_once();
        let lightweight = Lightweight::new(13567.0);
        let shiploads = Shiploads::new(vec![
            Shipload::new(140.2, Point::new(40.23, 0.0, 0.0), 15.21),
            Shipload::new(150.0, Point::new(40.0, 0.0, 0.0), 25.0),
            Shipload::new(150.0, Point::new(40.0, 0.0, 0.0), 20.0),
            Shipload::new(140.2, Point::new(30.23, 0.0, 0.0), 15.21),
            Shipload::new(150.0, Point::new(20.0, 0.0, 0.0), 25.0),
            Shipload::new(150.0, Point::new(10.0, 0.0, 0.0), 20.0),
        ]);
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let lightweight_intensity =
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight);
        let deadweight_intensity = DeadweightIntensity::new(&shiploads, ship_dimensions);
        let displacement_intensity =
            DisplacementIntensity::new(deadweight_intensity, lightweight_intensity);
        let lcb = LCG::new(displacement_intensity);
        assert_eq!(0.69, lcb.lcg().my_round(2));
    }
}
