#[cfg(test)]
mod tests {
    use crate::{
        core::{round::Round, water_density::WaterDensity},
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
            let _ = env_logger::try_init();
        })
    }

    #[test]
    fn empty_ship_buoyancy_intensity_ok_test() {
        // Силы поддержания судна в порожнем состоянии.
        call_once();
        let frames_file = "src/tests/unit/strength/test_data/frames.json".to_string();
        let shiploads_file = "src/tests/unit/strength/test_data/empty_ship.json".to_string();
        let frames = Frames::from_json_file(frames_file).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let shiploads = Rc::new(Shiploads::from_json_file(shiploads_file).unwrap());
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let d_t = Rc::new(DisplacementTonnage::new(
            lightweight,
            Rc::new(Deadweight::new(shiploads.clone())),
        ));
        let ship_trimming = ShipTrimming::new(
            Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions)),
            Rc::new(Displacement::new(
                bonjean_scale.clone(),
                ship_dimensions,
                WaterDensity::new(1.025),
            )),
            Rc::new(LCG::new(
                Rc::new(DisplacementIntensity::new(
                    Rc::new(DeadweightIntensity::new(shiploads, ship_dimensions)),
                    Rc::new(LightweightIntensity::from_ship_input_data(
                        ship_dimensions,
                        lightweight,
                    )),
                    ship_dimensions,
                )),
                ship_dimensions,
            )),
            d_t.clone(),
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let buoyancy_intensity =
            BuoyancyIntensity::new(ship_trimming, bonjean_scale, WaterDensity::new(1.025));
        let buoyancy_intensity_v = buoyancy_intensity
            .buoyancy_intensity(&ship_dimensions)
            .unwrap();
        let total_buoyancy = buoyancy_intensity_v.integral();
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
        let frames = Frames::from_json_file(frames_file).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let shiploads = Rc::new(Shiploads::from_json_file(shiploads_file).unwrap());
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let d_t = Rc::new(DisplacementTonnage::new(
            lightweight,
            Rc::new(Deadweight::new(shiploads.clone())),
        ));
        let ship_trimming = ShipTrimming::new(
            Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions)),
            Rc::new(Displacement::new(
                bonjean_scale.clone(),
                ship_dimensions,
                WaterDensity::new(1.025),
            )),
            Rc::new(LCG::new(
                Rc::new(DisplacementIntensity::new(
                    Rc::new(DeadweightIntensity::new(shiploads, ship_dimensions)),
                    Rc::new(LightweightIntensity::from_ship_input_data(
                        ship_dimensions,
                        lightweight,
                    )),
                    ship_dimensions,
                )),
                ship_dimensions,
            )),
            d_t.clone(),
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let buoyancy_intensity =
            BuoyancyIntensity::new(ship_trimming, bonjean_scale, WaterDensity::new(1.025));
        let buoyancy_intensity_v = buoyancy_intensity
            .buoyancy_intensity(&ship_dimensions)
            .unwrap();
        let total_buoyancy = buoyancy_intensity_v.integral();
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
