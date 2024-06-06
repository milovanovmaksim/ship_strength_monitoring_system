#[cfg(test)]
mod tests {
    use crate::{
        core::round::Round,
        strength::ship::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames},
            displacement::displacement::Displacement,
            ship_dimensions::ShipDimensions,
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
    fn displacement_ok_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/ship/bonjean_scale/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 2000000, 0.6);
        let bonjean_scale = BonjeanScale::new(&frames, ship_dimensions);
        let displacement = Displacement::new(&bonjean_scale, ship_dimensions);
        assert_eq!(
            14329.62,
            displacement.displacement(2.61, 2.61).unwrap().my_round(2)
        );
    }

    #[test]
    fn displacement_error_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/ship/bonjean_scale/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(&frames, ship_dimensions);
        let displacement = Displacement::new(&bonjean_scale, ship_dimensions);
        let ship_underwater_volume = displacement.displacement(2.61, 20.61);
        assert!(ship_underwater_volume.is_err());
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), ship_underwater_volume)
    }
}
