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
    fn ship_underwater_volume_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let ship_underwater_volume = bonjean_scale.ship_underwater_volume(2.61, 2.61).unwrap().my_round(2);
        assert_eq!(14329.62, ship_underwater_volume);
    }
}