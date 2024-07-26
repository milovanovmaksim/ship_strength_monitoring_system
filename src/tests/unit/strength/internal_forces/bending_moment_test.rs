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
            internal_forces::{
                bending_moment::BendingMoment, internal_force::InternalForce,
                share_force::ShareForce,
            },
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
    fn empty_bending_moment_ok_test() {
        // Судно порожнем.

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
        let d_i = Rc::new(DisplacementIntensity::new(
            Rc::new(DeadweightIntensity::new(shiploads.clone(), ship_dimensions)),
            Rc::new(LightweightIntensity::from_ship_input_data(
                ship_dimensions,
                lightweight,
            )),
            ship_dimensions,
        ));
        let bending_moment =
            BendingMoment::new(Rc::new(ShareForce::new(Rc::new(TotalShipload::new(
                d_i.clone(),
                Rc::new(BuoyancyIntensity::new(
                    Rc::new(Draft::new(
                        Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions)),
                        Rc::new(Displacement::new(
                            bonjean_scale.clone(),
                            ship_dimensions,
                            WaterDensity::new(1.025),
                        )),
                        Rc::new(LCG::new(d_i, ship_dimensions)),
                        d_t,
                        HydrostaticCurves::from_json_file(file_path).unwrap(),
                    )),
                    bonjean_scale,
                    WaterDensity::new(1.025),
                )),
            )))))
            .internal_force(&ship_dimensions)
            .unwrap();
        let max_bending_moment = bending_moment.max().unwrap();
        let last_bending_moment = bending_moment.last().unwrap().f_x2().abs();
        assert!(last_bending_moment / max_bending_moment <= 0.05); // Отношение взято из [Я.И Короткин Прочность корабля].
    }

    #[test]
    fn full_bending_moment_ok_test() {
        // Судно в грузу.
        call_once();

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
        let d_i = Rc::new(DisplacementIntensity::new(
            Rc::new(DeadweightIntensity::new(shiploads.clone(), ship_dimensions)),
            Rc::new(LightweightIntensity::from_ship_input_data(
                ship_dimensions,
                lightweight,
            )),
            ship_dimensions,
        ));
        let bending_moment =
            BendingMoment::new(Rc::new(ShareForce::new(Rc::new(TotalShipload::new(
                d_i.clone(),
                Rc::new(BuoyancyIntensity::new(
                    Rc::new(Draft::new(
                        Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions)),
                        Rc::new(Displacement::new(
                            bonjean_scale.clone(),
                            ship_dimensions,
                            WaterDensity::new(1.025),
                        )),
                        Rc::new(LCG::new(d_i, ship_dimensions)),
                        d_t,
                        HydrostaticCurves::from_json_file(file_path).unwrap(),
                    )),
                    bonjean_scale,
                    WaterDensity::new(1.025),
                )),
            )))))
            .internal_force(&ship_dimensions)
            .unwrap();
        let max_bending_moment = bending_moment.max().unwrap();
        let last_bending_moment = bending_moment.last().unwrap().f_x2().abs();
        assert!(last_bending_moment / max_bending_moment <= 0.05); // Отношение взято из [Я.И Короткин Прочность корабля].}
    }
}
