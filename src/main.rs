mod core;
mod cross_section_properties;
mod strength;
mod tests;
use core::visualisation::Visualisation;
use std::env;

use strength::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
    load::shiploads::Shiploads,
    ship::ship_dimensions::ShipDimensions
};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let lightweight = Lightweight::new(13550.0);
    let shiploads = Shiploads::from_json_file("./input_data/input_data.json".to_string()).unwrap();
    let ship_dimensions =
        ShipDimensions::from_json_file("./input_data/input_data.json".to_string()).unwrap();
    let deadweight_intnesity = DeadweightIntensity::new(&shiploads, ship_dimensions.clone());
    let deadweight_intnesity_spatium_functions = deadweight_intnesity.deadweight_intensity();
    let visualization = Visualisation::new(
        &deadweight_intnesity_spatium_functions,
        "Deadweight intnesity".to_string(),
        "Deadweight intnesity".to_string(),
        11.75,
    );
    visualization.visualize();
    let lightweight_intensity = LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lightweight);
    let lightweight_intensity_spatium_functions = lightweight_intensity.lightweight_intensity();
    let visualization = Visualisation::new(
        &lightweight_intensity_spatium_functions,
        "Lightweight intnesity".to_string(),
        "Lightweight intnesity".to_string(),
        11.75,
    );
    visualization.visualize();

    let displacement = DisplacementIntensity::new(deadweight_intnesity, lightweight_intensity);
    let spatium_functions = displacement.spatium_functions();
    let visualization = Visualisation::new(
        &spatium_functions,
        "Displacement intnesity".to_string(),
        "Displacement intnesity".to_string(),
        11.75,
    );
    visualization.visualize();
}

// Strength {
//     BendingMoment {
//         Sheareforce {
//             TotalShipload {
//                 BuoyancyLoadintensity {
//                     ShipTrimming {
//                         LCB{
//                             BonjeanScale{
//                                 Frames
//                             }
//                         },
//                         Displacement {
//                             BonjeanScale{
//                                 Frames
//                             }
//                         },
//                         LCG {
//                             DisplacementIntensity {
//                                 DeadweightIntensity {},
//                                 LightweightIntensity {}
//                             }
//                         },
//                         DisplacementTonnage{
//                             Lightweight{},
//                             Deadweight{}
//                         },
//                     }
//                 }
//             }
//         }
//     }
// }
