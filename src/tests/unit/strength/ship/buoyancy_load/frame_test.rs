#[cfg(test)]
mod tests {
    use crate::strength::ship::buoyancy_load::{bonjean_scale_data_type::BonjeanScaleDataType, frame::Frame};


    #[test]
    fn area_by_draft_ok_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts.clone(), areas.clone(), volumes, masses, abscissa).unwrap();
        assert_eq!(0.0, frame.data_by_draft(0.5, BonjeanScaleDataType::Area).unwrap());
        for i in 0..drafts.len() {
            let draft = *drafts.get(i).unwrap();
            assert_eq!(*areas.get(i).unwrap(), frame.data_by_draft(draft, BonjeanScaleDataType::Area).unwrap());
        }
        // Линейно интерполирует погруженную площадь шпангоута между осадками 2.0 и 3.0 метра.
        assert_eq!(81.605, frame.data_by_draft(2.5, BonjeanScaleDataType::Area).unwrap());
    }


    #[test]
    fn area_by_draft_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa).unwrap();
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.data_by_draft(15.0, BonjeanScaleDataType::Area));
    }


    #[test]
    fn volume_by_draft_ok_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts.clone(), areas, volumes.clone(), masses, abscissa).unwrap();
        assert_eq!(0.0, frame.data_by_draft(0.5, BonjeanScaleDataType::Volume).unwrap());
        for i in 0..drafts.len() {
            let draft = *drafts.get(i).unwrap();
            assert_eq!(*volumes.get(i).unwrap(), frame.data_by_draft(draft, BonjeanScaleDataType::Volume).unwrap());
        }
        // Линейно интерполирует погруженный объем шпангоута между осадками 2.0 и 3.0 метра.
        assert_eq!(958.855, frame.data_by_draft(2.5, BonjeanScaleDataType::Volume).unwrap());
    }

    #[test]
    fn volume_by_draft_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa).unwrap();
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.data_by_draft(15.0, BonjeanScaleDataType::Volume));
    }


    #[test]
    fn massa_by_draft_ok_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts.clone(), areas, volumes, masses.clone(), abscissa).unwrap();
        assert_eq!(0.0, frame.data_by_draft(0.5, BonjeanScaleDataType::Massa).unwrap());
        for i in 0..drafts.len() {
            let draft = *drafts.get(i).unwrap();
            assert_eq!(*masses.get(i).unwrap(), frame.data_by_draft(draft, BonjeanScaleDataType::Massa).unwrap());
        }
        // Линейно интерполирует погруженную массу шпангоута между осадками 2.0 и 3.0 метра.
        assert_eq!(958.855, frame.data_by_draft(2.5, BonjeanScaleDataType::Massa).unwrap());
    }

    #[test]
    fn massa_by_draft_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa).unwrap();
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.data_by_draft(15.0, BonjeanScaleDataType::Massa));
    }

    #[test]
    fn frame_ok_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert!(frame.is_ok())
    }

    #[test]
    fn frame_drafts_empty_err_test() {
        let id = 6;
        let drafts = vec![];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert!(frame.is_err());
        assert_eq!("Вектор, содержащий осадки судна, не может быть пустым.".to_string(), frame.unwrap_err())
    }

    #[test]
    fn frame_areas_empty_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert!(frame.is_err());
        assert_eq!("Вектор, содержащий погруженные площади шпангоута от осадки, не может быть пустым".to_string(), frame.unwrap_err())
    }

    #[test]
    fn frame_volumes_empty_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert!(frame.is_err());
        assert_eq!("Вектор, содержащий погруженные объемы шпангоута от осадки, не может быть пустым".to_string(), frame.unwrap_err())
    }

    #[test]
    fn frame_masses_empty_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert!(frame.is_err());
        assert_eq!("Вектор, содержащий погруженные массы шпангоута от осадки, не может быть пустым.".to_string(), frame.unwrap_err())
    }

    #[test]
    fn same_len_input_data_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert!(frame.is_err());
        assert_eq!("Длины векторов, содержащих данные масштаба Бонжана для шпангоута, должны быть одинаковыми".to_string(), frame.unwrap_err())
    }
}