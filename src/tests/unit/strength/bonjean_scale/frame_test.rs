#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use crate::strength::bonjean_scale::frame::Frame;

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
    fn area_by_draft_ok_test() {
        let id = 6;
        let drafts = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
        ];
        let areas = vec![
            32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08,
            395.08, 428.08, 437.98,
        ];
        let abscissa = -25.0;
        let frame = Frame::new(id, drafts.clone(), areas.clone(), abscissa).unwrap();
        assert_eq!(0.0, frame.area_by_draft(0.5).unwrap());
        for i in 0..drafts.len() {
            let draft = *drafts.get(i).unwrap();
            assert_eq!(*areas.get(i).unwrap(), frame.area_by_draft(draft).unwrap());
        }
        // Линейно интерполирует погруженную площадь шпангоута между осадками 2.0 и 3.0 метра.
        assert_eq!(81.605, frame.area_by_draft(2.5).unwrap());
    }

    #[test]
    fn area_by_draft_err_test() {
        call_once();
        let id = 6;
        let drafts = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
        ];
        let areas = vec![
            32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08,
            395.08, 428.08, 437.98,
        ];
        let abscissa = -25.0;
        let frame = Frame::new(id, drafts, areas, abscissa).unwrap();
        assert_eq!(
            Err("Осадка превысила осадку судна в грузу.".to_string()),
            frame.area_by_draft(15.0)
        );
    }

    #[test]
    fn frame_ok_test() {
        let id = 6;
        let drafts = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
        ];
        let areas = vec![
            32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08,
            395.08, 428.08, 437.98,
        ];
        let abscissa = -25.0;
        let frame = Frame::new(id, drafts, areas, abscissa);
        assert!(frame.is_ok())
    }

    #[test]
    fn frame_drafts_empty_err_test() {
        let id = 6;
        let drafts = vec![];
        let areas = vec![
            32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08,
            395.08, 428.08, 437.98,
        ];
        let abscissa = -25.0;
        let frame = Frame::new(id, drafts, areas, abscissa);
        assert!(frame.is_err());
        assert_eq!(
            "Вектор, содержащий осадки судна, не может быть пустым.".to_string(),
            frame.unwrap_err()
        )
    }

    #[test]
    fn frame_areas_empty_err_test() {
        let id = 6;
        let drafts = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
        ];
        let areas = vec![];
        let abscissa = -25.0;
        let frame = Frame::new(id, drafts, areas, abscissa);
        assert!(frame.is_err());
        assert_eq!(
            "Вектор, содержащий погруженные площади шпангоута от осадки, не может быть пустым"
                .to_string(),
            frame.unwrap_err()
        )
    }

    #[test]
    fn same_len_input_data_err_test() {
        let id = 6;
        let drafts = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3,
        ];
        let areas = vec![
            65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08,
            428.08,
        ];
        let abscissa = -25.0;
        let frame = Frame::new(id, drafts, areas, abscissa);
        assert!(frame.is_err());
        assert_eq!("Длины векторов, содержащих данные масштаба Бонжана для шпангоута, должны быть одинаковыми".to_string(), frame.unwrap_err())
    }
}
