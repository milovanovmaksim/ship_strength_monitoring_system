#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use log::debug;

    use crate::{
        core::round::Round,
        strength::hydrostatic_curves::{
            hydrostatic_curves::HydrostaticCurves, hydrostatic_typedata::HydrostaticTypeData,
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
    fn get_data_by_draft_ok_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/ship/hydrostatic_curves/test_data/hydrostatic_curves.json"
                .to_string();
        let hidrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        assert_eq!(
            -11.3,
            hidrostatic_curves
                .get_data_by_draft(2.0, HydrostaticTypeData::LCB)
                .unwrap()
        );
        assert_eq!(
            -12.04,
            hidrostatic_curves
                .get_data_by_draft(2.0, HydrostaticTypeData::LCF)
                .unwrap()
        );
        assert_eq!(
            5658.69,
            hidrostatic_curves
                .get_data_by_draft(2.0, HydrostaticTypeData::WaterlineArea)
                .unwrap()
        );
        assert_eq!(
            1446.69,
            hidrostatic_curves
                .get_data_by_draft(2.0, HydrostaticTypeData::LongitudinalMetacentricRadius)
                .unwrap()
                .my_round(2)
        );
    }

    #[test]
    fn get_data_by_draft_error_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/ship/hydrostatic_curves/test_data/hydrostatic_curves.json"
                .to_string();
        let hidrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        let value = hidrostatic_curves.get_data_by_draft(20.1, HydrostaticTypeData::LCB);
        assert!(value.is_err());
        assert_eq!(Err("Осадка превысила максимальную осадку для данного судна. Максимальная осадка по гидростатическим кривым составляет: 13.3, передано значение: 20.1".to_string()), value);
    }

    #[test]
    fn draft_by_displacement_tonnage_ok_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/ship/hydrostatic_curves/test_data/hydrostatic_curves.json"
                .to_string();
        let hidrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        let value = hidrostatic_curves
            .draft_by_displacement_tonnage(5605.2)
            .unwrap();
        assert_eq!(1.0, value);
        let value = hidrostatic_curves
            .draft_by_displacement_tonnage(1.0)
            .unwrap();
        assert_eq!(0.0, value);
    }

    #[test]
    fn draft_by_displacement_tonnage_error_test() {
        call_once();
        let file_path =
            "src/tests/unit/strength/ship/hydrostatic_curves/test_data/hydrostatic_curves.json"
                .to_string();
        let hidrostatic_curves = HydrostaticCurves::from_json_file(file_path).unwrap();
        let value = hidrostatic_curves.draft_by_displacement_tonnage(85365.01);
        assert!(value.is_err());
        assert_eq!(Err("Весовое водоизмещение превысило максимальное водоизмещение для данного судна. Максимальное весовое водоизмещение по гидростатическим кривым составляет: 85365, передано значение: 85365.01".to_string()), value);
    }
}
