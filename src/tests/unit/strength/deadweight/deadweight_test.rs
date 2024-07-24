#[cfg(test)]
mod tests {
    use crate::{
        core::point::Point,
        strength::{
            deadweight::deadweight::Deadweight,
            displacement::displacement_tonnage::DisplacementTonnage,
            lightweight::lightweight::Lightweight,
            load::{shipload::Shipload, shiploads::Shiploads},
        },
    };
    use std::{env, rc::Rc, sync::Once};

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
    fn deadweight_test() {
        call_once();
        let dw = Rc::new(Deadweight::new(Rc::new(Shiploads::new(vec![
            Shipload::new(4.2, Point::new(25.23, 0.0, 0.0), 10.21),
            Shipload::new(5.0, Point::new(64.0, 0.0, 0.0), 1.0),
            Shipload::new(5.0, Point::new(-64.0, 0.0, 0.0), 1.0),
        ]))));
        assert_eq!(14.2, dw.deadweight())
    }
}
