#[cfg(test)]
mod tests {
    use crate::{
        core::{point::Point, round::Round},
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
            buoyancy_load::{lcg::LCG, ship_trimming::ShipTrimming},
            deadweight::{deadweight::Deadweight, deadweight_intensity::DeadweightIntensity},
            displacement::{
                displacement::Displacement, displacement_intensity::DisplacementIntensity,
                displacement_tonnage::DisplacementTonnage,
            },
            hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::{shipload::Shipload, shiploads::Shiploads},
            ship::ship_dimensions::ShipDimensions,
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
    fn trim_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::new(vec![
            Shipload::new(0.0, Point::new(40.23, 0.0, 0.0), 15.21),
            Shipload::new(0.0, Point::new(40.0, 0.0, 0.0), 25.0),
            Shipload::new(0.0, Point::new(40.0, 0.0, 0.0), 20.0),
            Shipload::new(0.0, Point::new(30.23, 0.0, 0.0), 15.21),
            Shipload::new(0.0, Point::new(20.0, 0.0, 0.0), 25.0),
            Shipload::new(0.0, Point::new(10.0, 0.0, 0.0), 20.0),
        ]);
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(&bonjean_scale, ship_dimensions.clone()),
            LCG::new(DisplacementIntensity::new(
                DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
                LightweightIntensity::new(ship_dimensions.clone(), Lightweight::new(13567.0)),
            )),
            DisplacementTonnage::new(Lightweight::new(13567.0), Deadweight::new(&shiploads)),
            HydrostaticCurves::from_json_file(file_path).unwrap(),
            ship_dimensions.clone(),
            &bonjean_scale,
        );
        let (aft_draft, nose_draft) = ship_trimming.trim().unwrap();
        assert_eq!(
            (1.34, 3.34),
            (aft_draft.my_round(2), nose_draft.my_round(2))
        );
    }
}
