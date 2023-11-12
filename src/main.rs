mod strength;
mod core;
mod tests;
mod cross_section_properties;
use core::visualisation::Visualisation;
use std::env;

use log::debug;
use strength::ship::{ship_dimensions::ShipDimensions, deadweight::deadweight::Deadweight, load::{shiploads::Shiploads, load_sharing::LoadSharing}};

use crate::strength::ship::deadweight::deadweight_intensity::DeadweightIntensity;



fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let shiploads = Shiploads::from_json_file("./input_data/data.json".to_string()).unwrap();
    let ship_dimensions = ShipDimensions::new(125.0, 20, 0.6);
    let deadweight_intnesity = DeadweightIntensity::new(shiploads, ship_dimensions);
    let spatium_functions = deadweight_intnesity.spatium_functions();
    let visualization = Visualisation::new(spatium_functions, "Deadweight intnesity".to_string(), "Deadweight intnesity".to_string(), 5.43);
    visualization.visualize();
}
