#[cfg(test)]
mod tests {
    use crate::{
        core::{round::Round, water_density::WaterDensity},
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames},
            displacement::displacement::Displacement,
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
    fn displacement_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.8);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let displacement =
            Displacement::new(bonjean_scale, ship_dimensions, WaterDensity::new(1.025));
        assert_eq!(
            80300.85,
            displacement
                .displacement_by_drafts(13.3, 13.3)
                .unwrap()
                .my_round(2)
        );
    }

    #[test]
    fn displacement_error_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let displacement =
            Displacement::new(bonjean_scale, ship_dimensions, WaterDensity::new(1.0));
        let ship_underwater_volume = displacement.displacement_by_drafts(2.61, 20.61);
        assert!(ship_underwater_volume.is_err());
        assert_eq!(
            Err("Осадка превысила осадку судна в грузу.".to_string()),
            ship_underwater_volume
        )
    }
}
