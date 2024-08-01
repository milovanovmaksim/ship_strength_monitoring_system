#[cfg(test)]
mod tests {
    use crate::strength::{
        deadweight::deadweight::Deadweight,
        displacement::displacement_tonnage::DisplacementTonnage,
        lightweight::lightweight::Lightweight,
        load::{shipload::Shipload, shiploads::Shiploads},
    };
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
    fn displacement_tonnage_test() {
        call_once();
        let d_t = DisplacementTonnage::new(
            Lightweight::new(10.0),
            Deadweight::from_shiplods(&Shiploads::new(vec![Shipload::new(
                10.0,
                crate::core::point::Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                10.0,
            )])),
        );

        assert_eq!(20.0, d_t.displacement_tonnage());
    }
}
