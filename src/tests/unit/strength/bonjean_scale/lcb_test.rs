#[cfg(test)]
mod tests {
    use crate::{
        core::round::Round,
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
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
    fn lcb_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let lcb = LCB::new(&bonjean_scale, ship_dimensions);
        assert_eq!(-11.29, lcb.lcb(2.61, 2.61).unwrap().my_round(2));
    }

    #[test]
    fn lcb_error_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let lcb = LCB::new(&bonjean_scale, ship_dimensions);
        let xc = lcb.lcb(2.61, 20.61);
        assert!(xc.is_err());
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), xc)
    }
}
