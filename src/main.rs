mod strength;
mod core;
mod tests;
mod cross_section_properties;
use core::visualisation::Visualisation;
use std::env;

use log::debug;
use strength::ship::{deadweight::deadweight::Deadweight, displacement::displacement_intensity::DisplacementIntensity, lightweight::lightweight_intensity::LightweightIntensity, load::shiploads::Shiploads, ship_dimensions::ShipDimensions};

use crate::strength::ship::deadweight::deadweight_intensity::DeadweightIntensity;



fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let shiploads = Shiploads::from_json_file("./input_data/input_data.json".to_string()).unwrap();

    let ship_dimensions = ShipDimensions::from_json_file("./input_data/input_data.json".to_string()).unwrap();
    let mut lightweight_intensity = LightweightIntensity::new();
    let spatium_function = lightweight_intensity.spatium_functions(1750.56, &ship_dimensions);
    let visualization = Visualisation::new(&spatium_function, "Lightweight intnesity".to_string(), "Lightweight intnesity".to_string(), 6.25);
    visualization.visualize();
}
