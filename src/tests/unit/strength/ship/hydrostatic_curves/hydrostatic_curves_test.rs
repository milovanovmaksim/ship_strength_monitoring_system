#[cfg(test)]
mod tests {
    use std::{env, sync::Once};

    use log::debug;

    use crate::strength::ship::buoyancy_load::frames::Frames;

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
    fn get_data_by_draft_test() {
        todo!()
    }
}
