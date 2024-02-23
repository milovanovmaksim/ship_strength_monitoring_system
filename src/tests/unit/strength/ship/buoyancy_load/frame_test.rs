#[cfg(test)]
mod tests {
    use crate::strength::ship::buoyancy_load::frame::Frame;


    #[test]
    fn area_by_draft_ok_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert_eq!(0.0, frame.area_by_draft(0.5).unwrap());
        assert_eq!(32.3, frame.area_by_draft(1.0).unwrap());
        assert_eq!(65.12, frame.area_by_draft(2.0).unwrap());
        assert_eq!(437.98, frame.area_by_draft(13.3).unwrap());
        assert_eq!(428.08, frame.area_by_draft(13.0).unwrap());
        assert_eq!(81.605, frame.area_by_draft(2.5).unwrap());
    }


    #[test]
    fn area_by_draft_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.area_by_draft(15.0));
    }


    #[test]
    fn volume_by_draft_ok_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert_eq!(0.0, frame.volume_by_draft(0.5).unwrap());
        assert_eq!(379.52, frame.volume_by_draft(1.0).unwrap());
        assert_eq!(765.20, frame.volume_by_draft(2.0).unwrap());
        assert_eq!(5146.22, frame.volume_by_draft(13.3).unwrap());
        assert_eq!(5029.90, frame.volume_by_draft(13.0).unwrap());
        assert_eq!(958.855, frame.volume_by_draft(2.5).unwrap());
    }

    #[test]
    fn volume_by_draft_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.volume_by_draft(15.0));
    }


    #[test]
    fn massa_by_draft_ok_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert_eq!(0.0, frame.massa_by_draft(0.5).unwrap());
        assert_eq!(379.52, frame.massa_by_draft(1.0).unwrap());
        assert_eq!(765.20, frame.massa_by_draft(2.0).unwrap());
        assert_eq!(5146.22, frame.massa_by_draft(13.3).unwrap());
        assert_eq!(5029.90, frame.massa_by_draft(13.0).unwrap());
        assert_eq!(958.855, frame.massa_by_draft(2.5).unwrap());
    }

    #[test]
    fn massa_by_draft_err_test() {
        let id = 6;
        let drafts = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 13.3];
        let areas = vec![32.30, 65.12, 98.09, 131.08, 164.08, 197.08, 230.08, 263.08, 296.08, 329.08, 362.08, 395.08, 428.08, 437.98];
        let volumes = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let masses = vec![379.52, 765.20, 1152.51, 540.16, 1927.90, 2315.65, 2703.40, 3091.15, 3478.90, 3866.65, 4254.40, 4642.15, 5029.90, 5146.22];
        let abscissa =  -25.0;
        let frame = Frame::new(id, drafts, areas, volumes, masses, abscissa);
        assert_eq!(Err("Осадка превысила максимально допустимое значение для данного судна. Максимальная осадка: 13.3 [м].".to_string()), frame.massa_by_draft(15.0));
    }
}