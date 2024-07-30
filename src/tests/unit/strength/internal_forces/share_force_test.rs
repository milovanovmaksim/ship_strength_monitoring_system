#[cfg(test)]
mod tests {

    use crate::{
        core::water_density::WaterDensity,
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
            buoyancy_intensity::{buoyancy_intensity::BuoyancyIntensity, draft::Draft, lcg::LCG},
            deadweight::{deadweight::Deadweight, deadweight_intensity::DeadweightIntensity},
            displacement::{
                displacement::Displacement, displacement_intensity::DisplacementIntensity,
                displacement_tonnage::DisplacementTonnage,
            },
            hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
            internal_forces::share_force::ShareForce,
            lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
            load::{shiploads::Shiploads, total_shipload::TotalShipload},
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
    fn empty_share_force_ok_test() {
        // Судно порожнем.
        call_once();

        let frames_file = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file = "src/tests/unit/strength/test_data/empty_ship.json".to_string();
        let hydrostatic_curves_file =
            "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lw = Lightweight::new(13550.0);
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.74);
        let lw_i = LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lw);
        let shiploads = Shiploads::from_json_file(shiploads_file).unwrap();
        let dw_i = DeadweightIntensity::builder(&shiploads, ship_dimensions).build();
        let disp_i = DisplacementIntensity::from_dw_i_and_lw_i(&dw_i, &lw_i).unwrap();
        let dw = Deadweight::from_shiplods(&shiploads);
        let d_t = DisplacementTonnage::new(lw, dw);
        let water_density = WaterDensity::new(1.025);
        let frames = Frames::from_json_file(frames_file).unwrap();
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let disp = Rc::new(Displacement::new(
            bonjean_scale.clone(),
            ship_dimensions,
            water_density,
        ));
        let lcb = Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions));
        let lcg = LCG::from_disp_i(&disp_i);
        let hydrostatic_curves =
            HydrostaticCurves::from_json_file(hydrostatic_curves_file).unwrap();
        let draft = Draft::new(lcb.clone(), disp.clone(), lcg, d_t, hydrostatic_curves);
        let b_i =
            BuoyancyIntensity::constructor(ship_dimensions, &draft, &bonjean_scale, water_density)
                .unwrap();
        let total_shipload = TotalShipload::from_disp_i_and_b_i(&disp_i, &b_i).unwrap();
        let share_force =
            ShareForce::from_total_ship_load(&total_shipload).with_correction(ship_dimensions);
        let max_share_force = share_force.share_force().max().unwrap();
        let last_share_force = share_force.share_force().last().unwrap().f_x2().abs();
        assert!(last_share_force / max_share_force <= 0.05); // Отношение взято из [Я.И Короткин Прочность корабля].
    }

    #[test]
    fn full_share_force_ok_test() {
        // Судно в грузу.
        call_once();

        let frames_file = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file = "src/tests/unit/strength/test_data/full_ship.json".to_string();
        let hydrostatic_curves_file =
            "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lw = Lightweight::new(13550.0);
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.74);
        let lw_i = LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lw);
        let shiploads = Shiploads::from_json_file(shiploads_file).unwrap();
        let dw_i = DeadweightIntensity::builder(&shiploads, ship_dimensions).build();
        let disp_i = DisplacementIntensity::from_dw_i_and_lw_i(&dw_i, &lw_i).unwrap();
        let dw = Deadweight::from_shiplods(&shiploads);
        let d_t = DisplacementTonnage::new(lw, dw);
        let water_density = WaterDensity::new(1.025);
        let frames = Frames::from_json_file(frames_file).unwrap();
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let disp = Rc::new(Displacement::new(
            bonjean_scale.clone(),
            ship_dimensions,
            water_density,
        ));
        let lcb = Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions));
        let lcg = LCG::from_disp_i(&disp_i);
        let hydrostatic_curves =
            HydrostaticCurves::from_json_file(hydrostatic_curves_file).unwrap();
        let draft = Draft::new(lcb.clone(), disp.clone(), lcg, d_t, hydrostatic_curves);
        let b_i =
            BuoyancyIntensity::constructor(ship_dimensions, &draft, &bonjean_scale, water_density)
                .unwrap();
        let total_shipload = TotalShipload::from_disp_i_and_b_i(&disp_i, &b_i).unwrap();
        let share_force =
            ShareForce::from_total_ship_load(&total_shipload).with_correction(ship_dimensions);
        let max_share_force = share_force.share_force().max().unwrap();
        let last_share_force = share_force.share_force().last().unwrap().f_x2().abs();
        assert!(last_share_force / max_share_force <= 0.05); // Отношение взято из [Я.И Короткин Прочность корабля].
    }
}
