#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use crate::{
        core::round::Round,
        strength::{
            bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames},
            ship::ship_dimensions::ShipDimensions,
        },
    };

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
    fn frame_underwater_volume_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let frame_underwater_volume = bonjean_scale.frame_underwater_volume(-58.75, 2.61).unwrap();
        let frame_underwater_volume2 = bonjean_scale
            .frame_underwater_volume_trapezoid(-58.75, 2.61)
            .unwrap();
        assert_eq!(977.62, frame_underwater_volume.my_round(2));
        assert_eq!(989.54, frame_underwater_volume2.my_round(2));
        let err = {
            if frame_underwater_volume > frame_underwater_volume2 {
                ((frame_underwater_volume - frame_underwater_volume2) / frame_underwater_volume2)
                    * 100.0
            } else if frame_underwater_volume2 > frame_underwater_volume {
                ((frame_underwater_volume2 - frame_underwater_volume) / frame_underwater_volume)
                    * 100.0
            } else {
                0.0
            }
        };
        assert!(err < 5.0);
    }

    #[test]
    fn frame_underwater_volume_draft_out_error_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let frame_underwater_volume = bonjean_scale.frame_underwater_volume(-58.75, 20.61);
        assert!(frame_underwater_volume.is_err());
        assert_eq!(
            Err("Осадка превысила осадку судна в грузу.".to_string()),
            frame_underwater_volume
        )
    }

    #[test]
    fn frame_underwater_volume_abscissa_out_error_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let frame_underwater_volume = bonjean_scale.frame_underwater_volume(-158.75, 2.61);
        assert!(frame_underwater_volume.is_err());
        assert_eq!(Err("Абсцисса вышла за пределы координаты кормы судна. Координа кормы: -117.5. Передано значение: -158.75".to_string()), frame_underwater_volume)
    }

    #[test]
    fn frame_underwater_area_ok_test() {
        call_once();
        let file_path = "src/tests/unit/strength/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let ship_dimensions = ShipDimensions::new(235.0, 20, 0.6);
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions);
        let frame_underwater_area = bonjean_scale.frame_underwater_area(-58.75, 2.61).unwrap();
        assert_eq!(83.2, frame_underwater_area.my_round(2));
    }
}
