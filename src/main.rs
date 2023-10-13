mod strength;
mod core;
mod tests;
mod cross_section_properties;
use std::env;

use strength::ship::{loads::{deadweight::deadweight::Deadweight, lightweight::lightweight::Lightweight}, ship_dimensions::ShipDimensions};



fn main() {
    env::set_var("RUST_LOG", "debug");
    //env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let lightweight = 17501.0;
    let ship_metrics = ShipDimensions::new(256.13,20, 0.78);
    let test_lightweight = Deadweight::from_json_file("./input_data/data.json".to_string()).unwrap();
    let output = test_lightweight.deadweight_intensity();
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
