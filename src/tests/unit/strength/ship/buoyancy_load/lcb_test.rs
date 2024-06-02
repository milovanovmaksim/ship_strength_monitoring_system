#[cfg(test)]
mod tests {
    use std::{env, sync::Once};
    use crate::{core::round::Round, strength::ship::{buoyancy_load::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
        ship_dimensions::ShipDimensions}};

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
    fn lcb_test() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(&frames, ship_dimensions);
        let lcb = LCB::new(&bonjean_scale, ship_dimensions);
        assert_eq!(-11.29, lcb.lcb(2.61, 2.61).unwrap().my_round(2))
    }

}