#[cfg(test)]
mod tests {
    use crate::strength::bonjean_scale::{frame::Frame, frames::Frames};
    use std::{env, sync::Once};

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
    fn existed_frame_by_abscissa_test() {
        call_once();
        let frames = Frames::from_json_file("./input_data/frames.json".to_string()).unwrap();

        let tested_frame = Frame::new(
            1,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
            ],
            vec![
                7.04, 18.87, 32.74, 47.74, 63.36, 79.34, 95.51, 111.69, 127.13, 140.31, 152.75,
                165.99, 180.14, 184.56,
            ],
            -105.75,
        )
        .unwrap();
        assert_eq!(
            (Some(&tested_frame), None),
            frames.frame_by_abscissa(-105.75)
        );
    }

    #[test]
    fn not_existed_frame_by_abscissa_test() {
        call_once();
        let frames = Frames::from_json_file("./input_data/frames.json".to_string()).unwrap();

        let left_frame = Frame::new(
            1,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
            ],
            vec![
                7.04, 18.87, 32.74, 47.74, 63.36, 79.34, 95.51, 111.69, 127.13, 140.31, 152.75,
                165.99, 180.14, 184.56,
            ],
            -105.75,
        )
        .unwrap();
        let right_frame = Frame::new(
            2,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
            ],
            vec![
                17.39, 38.43, 60.82, 83.98, 107.68, 131.80, 156.19, 180.57, 204.56, 228.55, 252.98,
                278.03, 303.65, 311.44,
            ],
            -94.0,
        )
        .unwrap();
        assert_eq!(
            (Some(&left_frame), Some(&right_frame)),
            frames.frame_by_abscissa(-95.0)
        );
        assert_eq!((None, None), frames.frame_by_abscissa(200.0));
    }

    #[test]
    fn first_test() {
        let frames = Frames::from_json_file("./input_data/frames.json".to_string()).unwrap();
        let first_frame = Frame::new(
            0,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
            ],
            vec![
                0.22, 1.94, 4.94, 8.69, 12.86, 17.18, 21.54, 25.88, 30.09, 33.76, 35.01, 36.16,
                37.79, 38.39,
            ],
            -117.5,
        )
        .unwrap();
        assert_eq!(&first_frame, frames.first());
    }

    #[test]
    fn last_test() {
        let frames = Frames::from_json_file("./input_data/frames.json".to_string()).unwrap();
        let first_frame = Frame::new(
            20,
            vec![11.0, 12.0, 13.0, 13.3],
            vec![0.09, 0.67, 2.58, 3.74],
            117.5,
        )
        .unwrap();
        assert_eq!(&first_frame, frames.last());
    }
}
