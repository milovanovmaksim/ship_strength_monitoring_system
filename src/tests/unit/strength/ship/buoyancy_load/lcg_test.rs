#[cfg(test)]
mod tests {
    use std::{env, sync::Once};
    use crate::{core::{point::Point, round::Round}, strength::ship::{buoyancy_load::lcg::LCG, load::{shipload::Shipload, shiploads::Shiploads}}};

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
    fn lcg_ok_test() {
        call_once();
        let shiploads = Shiploads::new(vec![
            Shipload::new(4.2, Point::new(25.23, 0.0, 0.0), 10.21),
            Shipload::new(5.0, Point::new(64.0, 0.0, 0.0), 1.0),
            Shipload::new(5.0, Point::new(-64.0, 0.0, 0.0), 1.0)
        ]);
        let lcb = LCG::new(&shiploads);
        assert_eq!(7.46, lcb.lcg().my_round(2));
    }


}