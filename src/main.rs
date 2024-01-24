mod strength;
mod core;
mod tests;
mod cross_section_properties;
use core::visualisation::Visualisation;
use std::env;

use log::debug;
use strength::ship::{deadweight::deadweight::Deadweight, displacement::displacement_intensity::DisplacementIntensity, lightweight::lightweight::LightweightIntensity, load::{shiploads::Shiploads, load_sharing::LoadSharing}, ship_dimensions::ShipDimensions};

use crate::strength::ship::deadweight::deadweight_intensity::DeadweightIntensity;



fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let shiploads = Shiploads::from_json_file("./input_data/input_data.json".to_string()).unwrap();
    let ship_dimensions = ShipDimensions::from_json_file("./input_data/input_data.json".to_string()).unwrap();
    let deadweight_intnesity = DeadweightIntensity::new(shiploads, ship_dimensions);
    let deadweight_intnesity_spatium_functions = deadweight_intnesity.spatium_functions();
    let visualization = Visualisation::new(&deadweight_intnesity_spatium_functions, "Deadweight intnesity".to_string(), "Deadweight intnesity".to_string(), 6.25);
    visualization.visualize();

    let ship_dimensions = ShipDimensions::from_json_file("./input_data/input_data.json".to_string()).unwrap();
    let lightweight_intensity = LightweightIntensity::new(1750.56, ship_dimensions);
    let lightweight_intensity_spatium_functions = lightweight_intensity.lightweight_intensity();
    let visualization = Visualisation::new(&lightweight_intensity_spatium_functions, "Lightweight intnesity".to_string(), "Lightweight intnesity".to_string(), 6.25);
    visualization.visualize();

    let displacement = DisplacementIntensity::new(&deadweight_intnesity_spatium_functions,&lightweight_intensity_spatium_functions);
    let spatium_functions = displacement.spatium_functions();
    let visualization = Visualisation::new(&spatium_functions, "Displacement intnesity".to_string(), "Displacement intnesity".to_string(), 6.25);
    visualization.visualize();

}
