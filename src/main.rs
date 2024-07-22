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
    let input_path = "./input_data/input_data.json".to_string();
    let shiploads_file = "input_data/empty_ship.json".to_string();
    let frames_file = "./input_data/frames.json".to_string();
    let strength = Strength::new_project(input_path, shiploads_file, frames_file);
}
