mod core;
mod cross_section_properties;
mod strength;
mod tests;
use core::{point::Point, visualisation::Visualisation};
use std::env;

use log::debug;
use strength::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
    load::{shipload::Shipload, shiploads::Shiploads},
    ship::{ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction, spatium_functions::SpatiumFunctions}
};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let lightweight = Lightweight::new(13567.0);
    let shiploads = Shiploads::new(vec![
        Shipload::new(0.0, Point::new(40.23, 0.0, 0.0), 15.21),
    ]);
    let ship_dimensions =
        ShipDimensions::from_json_file("./input_data/input_data.json".to_string()).unwrap();
    let deadweight_intnesity = DeadweightIntensity::new(&shiploads, ship_dimensions.clone());
    let deadweight_intnesity_spatium_functions = deadweight_intnesity.deadweight_intensity();
    let visualization = Visualisation::new(
        &deadweight_intnesity_spatium_functions,
        "Deadweight intnesity".to_string(),
        "Deadweight intnesity".to_string(),
        6.25,
    );
    visualization.visualize();
    let lightweight_intensity = SpatiumFunctions::new(vec![
        SpatiumFunction::from_id(0, &ship_dimensions, 15.05, 15.05),
        SpatiumFunction::from_id(1, &ship_dimensions, 20.46, 20.46),
        SpatiumFunction::from_id(2, &ship_dimensions, 8.09, 8.09),
        SpatiumFunction::from_id(3, &ship_dimensions, 8.86, 8.86),
        SpatiumFunction::from_id(4, &ship_dimensions, 9.63, 9.63),
        SpatiumFunction::from_id(5, &ship_dimensions, 10.4, 10.4),
        SpatiumFunction::from_id(6, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(7, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(8, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(9, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(10, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(11, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(12, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(13, &ship_dimensions, 11.17, 11.17),
        SpatiumFunction::from_id(14, &ship_dimensions, 10.59, 10.59),
        SpatiumFunction::from_id(15, &ship_dimensions, 9.83, 9.83),
        SpatiumFunction::from_id(16, &ship_dimensions, 11.36, 11.36),
        SpatiumFunction::from_id(17, &ship_dimensions, 22.4, 22.4),
        SpatiumFunction::from_id(18, &ship_dimensions, 17.7, 17.7),
        SpatiumFunction::from_id(19, &ship_dimensions, 13.19, 13.9),
    ]);
    let lightweight_intensity = LightweightIntensity::new(lightweight_intensity);
    let lightweight_intensity_spatium_functions = lightweight_intensity.lightweight_intensity();
    let visualization = Visualisation::new(
        &lightweight_intensity_spatium_functions,
        "Lightweight intnesity".to_string(),
        "Lightweight intnesity".to_string(),
        6.25,
    );
    visualization.visualize();

    let displacement = DisplacementIntensity::new(deadweight_intnesity, lightweight_intensity);
    let spatium_functions = displacement.spatium_functions();
    let visualization = Visualisation::new(
        &spatium_functions,
        "Displacement intnesity".to_string(),
        "Displacement intnesity".to_string(),
        6.25,
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
