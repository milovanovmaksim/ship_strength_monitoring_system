#[cfg(test)]
mod tests {
    use crate::{
        core::round::Round,
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
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
            let _ = tracing_subscriber::fmt().compact().try_init();
        })
    }

    #[test]
    fn lcb_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let lcb = LCB::new(bonjean_scale, ship_dimensions);
        assert_eq!(-4.23, lcb.lcb(2.34, 4.07).unwrap().my_round(2));
    }

    #[test]
    fn lcb_error_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let lcb = LCB::new(bonjean_scale, ship_dimensions);
        let xc = lcb.lcb(2.61, 20.61);
        assert!(xc.is_err());
        assert_eq!(
            Err("Осадка превысила осадку судна в грузу.".to_string()),
            xc
        )
    }
}
