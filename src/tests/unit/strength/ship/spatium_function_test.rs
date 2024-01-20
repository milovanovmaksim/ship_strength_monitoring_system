#[cfg(test)]
mod tests {
    use std::{sync::Once, env};
    use crate::{strength::ship::{ship_dimensions::ShipDimensions,
        load::{shipload::Shipload, load_sharing::LoadSharing}}, core::point::Point};



    static INIT: Once = Once::new();

    fn call_once() {
        INIT.call_once(|| {
                env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
                // env::set_var("RUST_BACKTRACE", "1");
                env::set_var("RUST_BACKTRACE", "full");
                env_logger::init();
            }
        )
    }

    
}