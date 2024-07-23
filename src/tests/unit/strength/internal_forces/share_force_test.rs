#[cfg(test)]
mod tests {

    use crate::{
        core::water_density::WaterDensity,
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
            internal_forces::{internal_force::InternalForce, share_force::ShareForce},
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
    fn empty_share_force_ok_test() {
        // Судно порожнем.

        let share_force = ShareForce::new(total_shipload)
            .internal_force(&ship_dimensions)
            .unwrap();
        let max_share_force = share_force.max().unwrap();
        let last_share_force = share_force.last().unwrap().f_x2().abs();
        assert!(last_share_force / max_share_force <= 0.05); // Отношение взято из [Я.И Короткин Прочность корабля].
    }

    #[test]
    fn full_share_force_ok_test() {
        // Судно в грузу.
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();

        let share_force = ShareForce::new(total_shipload)
            .internal_force(&ship_dimensions)
            .unwrap();
        let max_share_force = share_force.max().unwrap();
        let last_share_force = share_force.last().unwrap().f_x2().abs();
        assert!(last_share_force / max_share_force <= 0.05); // Отношение взято из [Я.И Короткин Прочность корабля].
    }
}
