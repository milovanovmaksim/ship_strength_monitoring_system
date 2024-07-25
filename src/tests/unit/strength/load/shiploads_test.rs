#[cfg(test)]
mod tests {
    use log::info;

    use crate::{
        core::{point::Point, round::Round},
        strength::{
            load::{shipload::Shipload, shiploads::Shiploads},
            ship::ship_dimensions::ShipDimensions,
        },
    };
    use std::{env, sync::Once};

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
    fn sum_test() {
        call_once();
        let shipoads = Shiploads::new(vec![
            Shipload::new(
                10.0,
                Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                10.0,
            ),
            Shipload::new(
                20.2,
                Point {
                    x: 10.0,
                    y: 0.0,
                    z: 0.0,
                },
                10.0,
            ),
        ]);
        assert_eq!(30.2, shipoads.sum());
    }

    #[test]
    fn shared_loads_test() {
        call_once();
        let shiplods = Shiploads::new(vec![Shipload::new(
            10.0,
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            120.3,
        )]);
        let ship_dimnsions = ShipDimensions::new(235.0, 20, 0.74);
        let shared_shiploads = shiplods.shared_shiploads(&ship_dimnsions);
        assert!(
            (0.95 <= 10.0 / shared_shiploads.sum().my_round(2)
                && 10.0 / shared_shiploads.sum().my_round(2) <= 1.05)
        );
        info!("{:?}", shared_shiploads);
        assert_eq!(
            vec![
                Shipload::new(
                    0.12,
                    Point {
                        x: -59.45,
                        y: 0.0,
                        z: 0.0
                    },
                    1.4
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: -52.88,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: -41.13,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: -29.38,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: -17.63,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: -5.88,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: 5.88,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: 17.63,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: 29.38,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: 41.13,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.98,
                    Point {
                        x: 52.88,
                        y: 0.0,
                        z: 0.0
                    },
                    11.75
                ),
                Shipload::new(
                    0.12,
                    Point {
                        x: 59.45,
                        y: 0.0,
                        z: 0.0
                    },
                    1.4
                )
            ],
            shared_shiploads
                .into_iter()
                .map(|f| f)
                .collect::<Vec<Shipload>>()
        );
    }
}
