#[cfg(test)]
mod tests {
    use crate::{
        core::{point::Point, round::Round, water_density::WaterDensity},
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
        // Удифферентовка судна в порожнем состоянии.

        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::new(vec![
            Shipload::from_id(0, &ship_dimensions, 164.8),
            Shipload::from_id(1, &ship_dimensions, 615.2),
            Shipload::from_id(2, &ship_dimensions, 670.1),
            Shipload::from_id(3, &ship_dimensions, 75.0),
            Shipload::from_id(4, &ship_dimensions, 75.0),
            Shipload::from_id(5, &ship_dimensions, 175.0),
            Shipload::from_id(6, &ship_dimensions, 125.0),
            Shipload::from_id(7, &ship_dimensions, 125.0),
            Shipload::from_id(8, &ship_dimensions, 135.0),
            Shipload::from_id(9, &ship_dimensions, 135.0),
            Shipload::from_id(10, &ship_dimensions, 135.0),
            Shipload::from_id(11, &ship_dimensions, 135.0),
            Shipload::from_id(12, &ship_dimensions, 135.0),
            Shipload::from_id(13, &ship_dimensions, 135.0),
            Shipload::from_id(14, &ship_dimensions, 175.0),
            Shipload::from_id(15, &ship_dimensions, 175.0),
            Shipload::from_id(16, &ship_dimensions, 295.0),
            Shipload::from_id(17, &ship_dimensions, 270.0),
            Shipload::from_id(18, &ship_dimensions, 300.0),
            Shipload::from_id(19, &ship_dimensions, 382.0),
        ]);
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(
                &bonjean_scale,
                ship_dimensions.clone(),
                WaterDensity::new(1.025),
            ),
            LCG::new(DisplacementIntensity::new(
                DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
                LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
            )),
            DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads)),
            HydrostaticCurves::from_json_file(file_path).unwrap(),
            ship_dimensions.clone(),
            WaterDensity::new(1.025),
        );
        let (aft_draft, nose_draft) = ship_trimming.trim().unwrap();
        assert_eq!(
            (3.14, 3.18),
            (aft_draft.my_round(2), nose_draft.my_round(2))
        );
    }
}
