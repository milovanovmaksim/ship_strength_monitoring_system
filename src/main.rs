mod strength;
mod core;
mod tests;
mod cross_section_properties;
use core::visualisation::Visualisation;
use std::env;

use log::debug;
use strength::ship::{ship_dimensions::ShipDimensions, deadweight::deadweight::Deadweight, load::shiploads::Shiploads};

use crate::strength::ship::deadweight::deadweight_intensity::DeadweightIntensity;



fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let shiploads = Shiploads::from_json_file("./input_data/data.json".to_string());
    let deadweight_intensity = DeadweightIntensity::new(shiploads);
    let intensity = deadweight_intensity.deadweight_intensity().unwrap();
    let spatium_length = 6.25;
    let visualisation = Visualisation::new(
        intensity, "Deadweght intensity".to_string(),
         "Deadweight intensity".to_string(), spatium_length);
    visualisation.visualize();

}
