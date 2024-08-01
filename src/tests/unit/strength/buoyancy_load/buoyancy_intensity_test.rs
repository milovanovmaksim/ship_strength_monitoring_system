#[cfg(test)]
mod tests {
    use crate::{
        core::{round::Round, water_density::WaterDensity},
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
            buoyancy_intensity::{buoyancy_intensity::BuoyancyIntensity, draft::Draft, lcg::LCG},
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
    use log::info;
    use std::{env, rc::Rc, sync::Once};

    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
            env::set_var("RUST_LOG", "debug"); // off / error / warn / info / debug / trace
                                               // env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_BACKTRACE", "full");
            let _ = tracing_subscriber::fmt().compact().try_init();
        })
    }

    #[test]
    fn empty_ship_buoyancy_intensity_ok_test() {
        // Силы поддержания судна в порожнем состоянии.
        call_once();
        let frames_file = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file = "src/tests/unit/strength/test_data/empty_ship.json".to_string();
        let hydrostatic_curves_file =
            "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lw = Lightweight::new(13550.0);
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
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
        let draft = Draft::new(lcb, disp, lcg, d_t, hydrostatic_curves);
        let b_i =
            BuoyancyIntensity::constructor(ship_dimensions, &draft, &bonjean_scale, water_density)
                .unwrap();
        let total_buoyancy = b_i.buoyancy_intensity().integral();
        let d_t_v = d_t.displacement_tonnage();
        let error = (((total_buoyancy.abs() - d_t_v).abs() / d_t_v.min(total_buoyancy.abs()))
            * 100.0)
            .my_round(2);
        info!("total_buoyancy = {total_buoyancy} т");
        info!("displacement_tonnage = {d_t_v} т");
        info!("error = {error} %");
        assert!(error <= 5.0); // Весовое водоизмещение судна и силы поддержания не должны отличаться более чем на 5 %.
    }

    #[test]
    fn full_ship_buoyancy_intensity_ok_test() {
        // Силы поддержания судна в полном грузу.
        call_once();
        let frames_file = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file = "src/tests/unit/strength/test_data/full_ship.json".to_string();
        let hydrostatic_curves_file =
            "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lw = Lightweight::new(13550.0);
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
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
        let draft = Draft::new(lcb, disp, lcg, d_t, hydrostatic_curves);
        let b_i =
            BuoyancyIntensity::constructor(ship_dimensions, &draft, &bonjean_scale, water_density)
                .unwrap();
        let total_buoyancy = b_i.buoyancy_intensity().integral();
        let d_t_v = d_t.displacement_tonnage();
        let error = (((total_buoyancy.abs() - d_t_v).abs() / d_t_v.min(total_buoyancy.abs()))
            * 100.0)
            .my_round(2);
        info!("total_buoyancy = {total_buoyancy} т");
        info!("displacement_tonnage = {d_t_v} т");
        info!("error = {error} %");
        assert!(error <= 5.0); // Весовое водоизмещение судна и силы поддержания не должны отличаться более чем на 5 %.
    }
}
