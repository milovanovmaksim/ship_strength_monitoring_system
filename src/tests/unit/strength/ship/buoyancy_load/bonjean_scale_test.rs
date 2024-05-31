#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use crate::{core::round::Round, strength::ship::{buoyancy_load::{bonjean_scale::BonjeanScale, frames::Frames}, ship_dimensions::ShipDimensions}};

    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                let _ = env_logger::try_init();
            }
        )
    }

    #[test]
    fn underwater_area_frame_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let underwater_area_frame = bonjean_scale.underwater_area_frame(-56.25, 1.0).unwrap().my_round(2);
        assert_eq!(7.04, underwater_area_frame);

        //// Линейно интерполирует погруженную площадь шпангоута между абсциссами -65.25 м и -50.0 м.
        let underwater_area_frame = bonjean_scale.underwater_area_frame(-51.05, 1.0).unwrap().my_round(2);
        assert_eq!(15.65, underwater_area_frame);
    }


    #[test]
    fn ship_underwater_volume_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let ship_underwater_volume = bonjean_scale.ship_underwater_volume(3.5, 5.0).unwrap().my_round(2);
        assert_eq!(22492.91, ship_underwater_volume);
    }
}