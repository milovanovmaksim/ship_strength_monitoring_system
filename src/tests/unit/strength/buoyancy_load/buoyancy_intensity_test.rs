#[cfg(test)]
mod tests {
    use log::info;

    use crate::{
        core::{point::Point, round::Round, water_density::WaterDensity},
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
    fn empty_ship_buoyancy_intensity_ok_test() {
        call_once();
        // Силы поддержания судна в порожнем состоянии.

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
        let dt = DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads));
        let dt_v = dt.displacement_tonnage();
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(&bonjean_scale, ship_dimensions.clone(), water_density),
            LCG::new(DisplacementIntensity::new(
                DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
                LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
            )),
            dt,
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let buoyancy_intensity =
            BuoyancyIntensity::new(ship_trimming, &bonjean_scale, water_density);
        let buoyancy_intensity_v = buoyancy_intensity
            .buoyancy_intensity(&ship_dimensions)
            .unwrap();
        let mut total_buoyancy = 0.0;
        for value in buoyancy_intensity_v.as_ref() {
            total_buoyancy += value.integral();
        }
        let error = (((total_buoyancy.abs() - dt_v).abs() / dt_v.min(total_buoyancy.abs()))
            * 100.0)
            .my_round(2);
        info!("total_buoyancy = {total_buoyancy} т");
        info!("displacement_tonnage = {dt_v} т");
        info!("error = {error} %");
        assert!(error <= 5.0); // Весовое водоизмещение судна и силы поддержания не должны отличаться более чем на 5 %.
    }

    #[test]
    fn full_ship_buoyancy_intensity_ok_test() {
        call_once();
        // Силы поддержания судна в полном грузу.

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
        let dt = DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads));
        let dt_v = dt.displacement_tonnage();
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(&bonjean_scale, ship_dimensions.clone(), water_density),
            LCG::new(DisplacementIntensity::new(
                DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
                LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
            )),
            dt,
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let buoyancy_intensity =
            BuoyancyIntensity::new(ship_trimming, &bonjean_scale, water_density);
        let buoyancy_intensity_v = buoyancy_intensity
            .buoyancy_intensity(&ship_dimensions)
            .unwrap();
        let mut total_buoyancy = 0.0;
        for value in buoyancy_intensity_v.as_ref() {
            total_buoyancy += value.integral();
        }
        let error = (((total_buoyancy.abs() - dt_v).abs() / dt_v.min(total_buoyancy.abs()))
            * 100.0)
            .my_round(2);
        info!("total_buoyancy = {total_buoyancy} т");
        info!("displacement_tonnage = {dt_v} т");
        info!("error = {error} %");
        assert!(error <= 5.0); // Весовое водоизмещение судна и силы поддержания не должны отличаться более чем на 5 %.
    }

    #[test]
    fn buoyancy_intensity_error_test() {
        // Судно перегружено.
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let shiploads = Shiploads::new(vec![Shipload::new(
            80000.0,
            Point::new(0.0, 0.0, 0.0),
            11.75,
        )]);
        let file_path = "src/tests/unit/strength/test_data/hydrostatic_curves.json".to_string();
        let lightweight = Lightweight::new(13550.0);
        let water_density = WaterDensity::new(1.025);
        let dt = DisplacementTonnage::new(lightweight, Deadweight::new(&shiploads));
        let ship_trimming = ShipTrimming::new(
            LCB::new(&bonjean_scale, ship_dimensions.clone()),
            Displacement::new(&bonjean_scale, ship_dimensions.clone(), water_density),
            LCG::new(DisplacementIntensity::new(
                DeadweightIntensity::new(&shiploads, ship_dimensions.clone()),
                LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight),
            )),
            dt,
            HydrostaticCurves::from_json_file(file_path).unwrap(),
        );
        let buoyancy_intensity =
            BuoyancyIntensity::new(ship_trimming, &bonjean_scale, water_density);
        let buoyancy_intensity_v = buoyancy_intensity.buoyancy_intensity(&ship_dimensions);
        assert!(buoyancy_intensity_v.is_err());
        assert_eq!(
            Err(
                "Весовое водоизмещение 93550 тонн превысило весовое водоизмещение судна в грузу."
                    .to_string()
            ),
            buoyancy_intensity_v
        );
    }
}
