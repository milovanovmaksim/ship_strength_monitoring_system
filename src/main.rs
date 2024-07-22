mod core;
mod cross_section_properties;
mod strength;
mod tests;
use std::env;
use strength::strength::Strength;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let strength = Strength::new_project();
}
