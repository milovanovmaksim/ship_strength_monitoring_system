#[cfg(test)]
mod tests {
    use log::info;

    use crate::{
        core::{visualisation::Visualisation, water_density::WaterDensity},
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
            buoyancy_intensity::{
                buoyancy_intensity::BuoyancyIntensity, lcg::LCG, ship_trimming::ShipTrimming,
            },
            deadweight::{deadweight::Deadweight, deadweight_intensity::DeadweightIntensity},
            displacement::{
                displacement::Displacement, displacement_intensity::DisplacementIntensity,
                displacement_tonnage::DisplacementTonnage,
            },
            hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::{shiploads::Shiploads, total_shipload::TotalShipload},
            share_force::ShareForce,
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
    fn empty_share_force_intensity_ok_test() {
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
        let lightweight = Lightweight::new(13550.0);
        let water_density = WaterDensity::new(1.025);
        let d_t = DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads));
        let d_i = DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        );
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(&bonjean_scale, ship_dimensions.clone(), water_density),
            LCG::new(d_i),
            d_t,
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let b_i = BuoyancyIntensity::new(ship_trimming, &bonjean_scale, water_density);
        let d_i = DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        );
        let total_shipload = TotalShipload::new(d_i, b_i);
        let share_force = ShareForce::new(total_shipload)
            .share_force(&ship_dimensions)
            .unwrap();
        let mut max_share_force = 0.0;
        for s_f in share_force.as_ref() {
            let max_value = s_f.f_x1().abs().max(s_f.f_x2().abs());
            if max_value > max_share_force {
                max_share_force = max_value;
            }
        }
        let last_share_force = share_force.last().unwrap().f_x2().abs();
        // let visualization = Visualisation::new(
        //     &share_force,
        //     "Share force".to_string(),
        //     "Share force".to_string(),
        //     11.75,
        // );
        // visualization.visualize();
        assert!(last_share_force / max_share_force <= 0.05);
    }
    #[test]
    fn full_share_force_intensity_ok_test() {
        // Судно в грузу.
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file =
            "src/tests/unit/strength/buoyancy_load/test_data/full_ship.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::from_json_file(shiploads_file).unwrap();
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let water_density = WaterDensity::new(1.025);
        let d_t = DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads));
        let d_i = DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        );
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(&bonjean_scale, ship_dimensions.clone(), water_density),
            LCG::new(d_i),
            d_t,
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let b_i = BuoyancyIntensity::new(ship_trimming, &bonjean_scale, water_density);
        let d_i = DisplacementIntensity::new(
            DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
            LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
        );
        let total_shipload = TotalShipload::new(d_i, b_i);
        let share_force = ShareForce::new(total_shipload)
            .share_force(&ship_dimensions)
            .unwrap();
        let mut max_share_force = 0.0;
        for s_f in share_force.as_ref() {
            let max_value = s_f.f_x1().abs().max(s_f.f_x2().abs());
            if max_value > max_share_force {
                max_share_force = max_value;
            }
        }
        // let visualization = Visualisation::new(
        //     &share_force,
        //     "Share force".to_string(),
        //     "Share force".to_string(),
        //     11.75,
        // );
        // visualization.visualize();
        let last_share_force = share_force.last().unwrap().f_x2().abs();
        assert!(last_share_force / max_share_force <= 0.05);
    }
}
