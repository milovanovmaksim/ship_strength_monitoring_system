#[cfg(test)]
mod tests {
    use std::{env, sync::Once};
    use log::debug;

    use crate::{core::{json_file::JsonFile, round::Round}, strength::ship::{buoyancy_load::{bonjean_scale::BonjeanScale, frame::Frame}, ship_dimensions::ShipDimensions}};

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


    fn frames(file_path: String) -> Result<Vec<Frame>, String> {
        let json = JsonFile::new(file_path);
        let content = json.content()?;
        serde_json::from_reader(content).map_err(|err| { err.to_string() })
    }

    #[test]
    fn underwater_area_frame_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = frames(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6, 1750.0);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions).unwrap();
        let underwater_area_frame = bonjean_scale.underwater_area_frame(-56.25, 1.0).unwrap();
        assert_eq!(7.04, underwater_area_frame);

        //// Линейно интерполирует погруженную площадь шпангоута между абсциссами -65.25 м и -50.0 м.
        let underwater_area_frame = bonjean_scale.underwater_area_frame(-51.05, 1.0).unwrap().my_round(2);
        assert_eq!(15.65, underwater_area_frame);
    }

}