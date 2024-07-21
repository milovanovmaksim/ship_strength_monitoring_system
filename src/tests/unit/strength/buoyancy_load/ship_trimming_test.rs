#[cfg(test)]
mod tests {
    use crate::{
        core::round::Round,
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
            buoyancy_intensity::lcg::LCG,
            deadweight::deadweight_intensity::DeadweightIntensity,
            displacement::displacement_intensity::DisplacementIntensity,
            draft::ship_trimming::ShipTrimming,
            hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::shiploads::Shiploads,
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
    fn trim_empty_ship_ok_test() {
        call_once();
        // Удифферентовка судна в порожнем состоянии.

        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file =
            "src/tests/unit/strength/buoyancy_load/test_data/empty_ship.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::from_json_file(shiploads_file).unwrap();
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let lcb = LCB::new(&bonjean_scale, ship_dimensions.clone());
        let lcg = LCG::new(DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        ));
        let hydrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        let ship_trimming = ShipTrimming::new(&lcb, &lcg, &hydrostatic_curves);
        let (aft_draft, nose_draft) = ship_trimming
            .trimming(2.08, &ship_dimensions)
            .unwrap()
            .unwrap();
        assert_eq!(
            (1.54, 2.74),
            (aft_draft.my_round(2), nose_draft.my_round(2))
        );
    }

    #[test]
    fn trim_full_ship_ok_test() {
        call_once();
        // Удифферентовка судна в полном грузу.
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file =
            "src/tests/unit/strength/buoyancy_load/test_data/full_ship.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::from_json_file(shiploads_file).unwrap();
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let lcb = LCB::new(&bonjean_scale, ship_dimensions.clone());
        let lcg = LCG::new(DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        ));
        let hydrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        let ship_trimming = ShipTrimming::new(&lcb, &lcg, &hydrostatic_curves);
        let (aft_draft, nose_draft) = ship_trimming
            .trimming(13.08, &ship_dimensions)
            .unwrap()
            .unwrap();
        assert_eq!(
            (12.97, 13.19),
            (aft_draft.my_round(2), nose_draft.my_round(2))
        );
    }
}
