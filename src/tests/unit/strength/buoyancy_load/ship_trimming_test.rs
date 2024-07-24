#[cfg(test)]
mod tests {
    use crate::{
        core::{round::Round, water_density::WaterDensity},
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
            buoyancy_intensity::{lcg::LCG, ship_trimming::ShipTrimming},
            deadweight::{deadweight::Deadweight, deadweight_intensity::DeadweightIntensity},
            displacement::{
                displacement::Displacement, displacement_intensity::DisplacementIntensity,
                displacement_tonnage::DisplacementTonnage,
            },
            hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::shiploads::Shiploads,
            ship::ship_dimensions::ShipDimensions,
        },
    };
    use std::{env, rc::Rc, sync::Once};

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
    fn trim_empty_ship_ok_test() {
        call_once();
        // Удифферентовка судна в порожнем состоянии.

        let frames_file = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file = "src/tests/unit/strength/test_data/empty_ship.json".to_string();
        let frames = Frames::from_json_file(frames_file).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let shiploads = Rc::new(Shiploads::from_json_file(shiploads_file).unwrap());
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let ship_trimming = ShipTrimming::new(
            Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions)),
            Rc::new(Displacement::new(
                bonjean_scale,
                ship_dimensions,
                WaterDensity::new(1.025),
            )),
            Rc::new(LCG::new(
                Rc::new(DisplacementIntensity::new(
                    Rc::new(DeadweightIntensity::new(shiploads.clone(), ship_dimensions)),
                    Rc::new(LightweightIntensity::from_ship_input_data(
                        ship_dimensions,
                        lightweight,
                    )),
                    ship_dimensions,
                )),
                ship_dimensions,
            )),
            Rc::new(DisplacementTonnage::new(
                lightweight,
                Rc::new(Deadweight::new(shiploads)),
            )),
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let (aft_draft, nose_draft) = ship_trimming.trim(&ship_dimensions).unwrap();
        assert_eq!(
            (2.34, 4.07),
            (aft_draft.my_round(2), nose_draft.my_round(2))
        );
    }

    #[test]
    fn trim_full_ship_ok_test() {
        call_once();
        // Удифферентовка судна в полном грузу.

        let frames_file = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file = "src/tests/unit/strength/test_data/full_ship.json".to_string();
        let frames = Frames::from_json_file(frames_file).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let shiploads = Rc::new(Shiploads::from_json_file(shiploads_file).unwrap());
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let ship_trimming = ShipTrimming::new(
            Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions)),
            Rc::new(Displacement::new(
                bonjean_scale,
                ship_dimensions,
                WaterDensity::new(1.025),
            )),
            Rc::new(LCG::new(
                Rc::new(DisplacementIntensity::new(
                    Rc::new(DeadweightIntensity::new(shiploads.clone(), ship_dimensions)),
                    Rc::new(LightweightIntensity::from_ship_input_data(
                        ship_dimensions,
                        lightweight,
                    )),
                    ship_dimensions,
                )),
                ship_dimensions,
            )),
            Rc::new(DisplacementTonnage::new(
                lightweight,
                Rc::new(Deadweight::new(shiploads)),
            )),
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let (aft_draft, nose_draft) = ship_trimming.trim(&ship_dimensions).unwrap();
        assert_eq!(
            (13.09, 13.23),
            (aft_draft.my_round(2), nose_draft.my_round(2))
        );
    }
}
