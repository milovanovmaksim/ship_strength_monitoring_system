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
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.74);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::new(vec![Shipload::new(
            2994.0,
            Point::new(-50.0, 0.0, 0.0),
            11.21,
        )]);
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13575.0);
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(
                &bonjean_scale,
                ship_dimensions.clone(),
                WaterDensity::new(1.0),
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
            (3.01, 2.98),
            (aft_draft.my_round(2), nose_draft.my_round(2))
        );
    }
}
