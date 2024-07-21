#[cfg(test)]
mod tests {
    use log::info;

    use crate::{
        core::water_density::WaterDensity,
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
            buoyancy_intensity::{buoyancy_intensity::BuoyancyIntensity, lcg::LCG},
            deadweight::{deadweight::Deadweight, deadweight_intensity::DeadweightIntensity},
            displacement::{
                displacement::Displacement, displacement_intensity::DisplacementIntensity,
                displacement_tonnage::DisplacementTonnage,
            },
            draft::{draft::Draft, ship_trimming::ShipTrimming},
            hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::{shiploads::Shiploads, total_shipload::TotalShipload},
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
    fn empty_total_shipload_intensity_ok_test() {
        // Судно порожнем.
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file =
            "src/tests/unit/strength/buoyancy_load/test_data/empty_ship.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::from_json_file(shiploads_file).unwrap();
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let hydrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        let lightweight = Lightweight::new(13550.0);
        let water_density = WaterDensity::new(1.025);
        let lcb = LCB::new(&bonjean_scale, ship_dimensions.clone());
        let lcg = LCG::new(DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        ));
        let disp_i = DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        );
        let d_t = DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads));
        let d_t_v = d_t.displacement_tonnage();
        let b_i = BuoyancyIntensity::new(
            Draft::new(
                Displacement::new(&bonjean_scale, ship_dimensions.clone(), water_density),
                ShipTrimming::new(&lcb, &lcg, &hydrostatic_curves),
                DisplacementTonnage::new(Lightweight::new(13550.0), Deadweight::new(&shiploads)),
                &hydrostatic_curves,
                &lcb,
                &lcg,
            ),
            &bonjean_scale,
            water_density,
        );
        let b_i_v = b_i.buoyancy_intensity(&ship_dimensions).unwrap();
        let total_shipload = TotalShipload::new(disp_i, b_i)
            .total_shipload(&ship_dimensions)
            .unwrap();
        let tested_integral_total_shipload = total_shipload.integral();
        let total_buoyancy = b_i_v.integral();
        let integral_total_shipload = (total_buoyancy.abs() - d_t_v).abs();
        let error = ((integral_total_shipload - tested_integral_total_shipload.abs()).abs()
            / integral_total_shipload.min(tested_integral_total_shipload.abs()))
            * 100.0;
        info!("error = {error} %");
        assert!(error <= 5.0);
    }

    #[test]
    fn full_total_shipload_intensity_ok_test() {
        // Судно в полном грузу.

        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file =
            "src/tests/unit/strength/buoyancy_load/test_data/full_ship.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::from_json_file(shiploads_file).unwrap();
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let hydrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        let lightweight = Lightweight::new(13550.0);
        let water_density = WaterDensity::new(1.025);
        let lcb = LCB::new(&bonjean_scale, ship_dimensions.clone());
        let lcg = LCG::new(DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        ));
        let disp_i = DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        );
        let d_t = DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads));
        let d_t_v = d_t.displacement_tonnage();
        let b_i = BuoyancyIntensity::new(
            Draft::new(
                Displacement::new(&bonjean_scale, ship_dimensions.clone(), water_density),
                ShipTrimming::new(&lcb, &lcg, &hydrostatic_curves),
                DisplacementTonnage::new(Lightweight::new(13550.0), Deadweight::new(&shiploads)),
                &hydrostatic_curves,
                &lcb,
                &lcg,
            ),
            &bonjean_scale,
            water_density,
        );
        let b_i_v = b_i.buoyancy_intensity(&ship_dimensions).unwrap();
        let total_shipload = TotalShipload::new(disp_i, b_i)
            .total_shipload(&ship_dimensions)
            .unwrap();
        let tested_integral_total_shipload = total_shipload.integral();
        let total_buoyancy = b_i_v.integral();
        let integral_total_shipload = (total_buoyancy.abs() - d_t_v).abs();
        let error = ((integral_total_shipload - tested_integral_total_shipload.abs()).abs()
            / integral_total_shipload.min(tested_integral_total_shipload.abs()))
            * 100.0;
        info!("error = {error} %");
        assert!(error <= 5.0);
    }
}
