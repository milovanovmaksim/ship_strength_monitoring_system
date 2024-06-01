#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use log::debug;

    use crate::strength::ship::buoyancy_load::frames::Frames;

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
    fn frame_underwater_volume_test() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let frame_underwater_volume = frames.frame_underwater_volume(50.7, 3.0, 11.75);
        assert!(frame_underwater_volume.is_ok());
        assert_eq!(963.43, frame_underwater_volume.unwrap());
    }

    #[test]
    fn frame_underwater_volume_draft_out_test_err() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let frame_underwater_volume = frames.frame_underwater_volume(50.7, 33.0, 11.75);
        assert!(frame_underwater_volume.is_err());
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame_underwater_volume);
    }

    #[test]
    fn frame_underwater_volume_abscissa_out_test_err() {
        call_once();
        let file_path = "src/tests/unit/strength/ship/buoyancy_load/test_data/frames.json".to_string();
        let frames = Frames::from_json_file(file_path).unwrap();
        let frame_underwater_volume = frames.frame_underwater_volume(500.7, 3.123, 11.75);
        assert!(frame_underwater_volume.is_err());
        assert_eq!(Err("Абсцисса вышла за пределы координаты носа судна. Координа носа: 117.5. Передано значение: 500.7".to_string()), frame_underwater_volume);
    }
}