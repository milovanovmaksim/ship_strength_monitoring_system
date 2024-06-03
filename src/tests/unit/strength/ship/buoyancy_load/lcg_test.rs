#[cfg(test)]
mod tests {
    use std::{env, sync::Once};
    use crate::{core::{point::Point, round::Round}, strength::ship::{buoyancy_load::lcg::LCG, deadweight::deadweight_intensity::DeadweightIntensity, displacement::displacement_intensity::DisplacementIntensity, lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
        load::{shipload::Shipload, shiploads::Shiploads}, ship_dimensions::ShipDimensions}};

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
    fn lcg_ok_test() {
        call_once();
        let lightweight = Lightweight::new(1357.6);
        let shiploads = Shiploads::new(vec![
            Shipload::new(4.2, Point::new(25.23, 0.0, 0.0), 10.21),
            Shipload::new(5.0, Point::new(64.0, 0.0, 0.0), 1.0),
            Shipload::new(5.0, Point::new(-64.0, 0.0, 0.0), 1.0)
        ]);
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let lightweight_intensity = LightweightIntensity::new(ship_dimensions.clone(), lightweight);
        let deadweight_intensity = DeadweightIntensity::new(shiploads, ship_dimensions);
        let displacement_intensity = DisplacementIntensity::new(deadweight_intensity, lightweight_intensity);
        let lcb = LCG::new(displacement_intensity);
        assert_eq!(-0.56, lcb.lcg().my_round(2));
    }


}