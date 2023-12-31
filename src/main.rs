mod strength;
mod core;
mod tests;
mod cross_section_properties;
use std::env;

use strength::ship::{ship_dimensions::ShipDimensions, deadweight::deadweight::Deadweight};

use crate::strength::ship::deadweight::deadweight_intensity::DeadweightIntensity;



fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let deadweight_intensity = DeadweightIntensity::from_json_file("./input_data/data.json".to_string()).unwrap();
    deadweight_intensity.deadweight_intensity();

}

// Solution{
//     EquBeam{
//         CrossSections::from_csv_file(),
//         BendingMoment{
//             SheareForce{
//                 TotalShipLoad {
//                     BouyanLoad{
//                         Ship::from_file(file_path),
//                         BonjeanScale::from_file(file_path)
//                     },
//                     Displacment{
//                         Deadweight::from_json_file(file_path),
//                         Lightweight::from_json_file(file_path)
//                     }
//                 }
//             }
//         }
//     }
// }
