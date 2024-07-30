mod core;
mod strength;
mod tests;
use core::visualisation::{DiagrammType, Visualisation};
use std::env;
use strength::strength::Strength;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");

    env_logger::init();
    let input_path = "./input_data/input_data.json".to_string();
    let shiploads_file = "input_data/full_ship.json".to_string();
    let frames_file = "./input_data/frames.json".to_string();
    let hydrostatic_curves = "./input_data/hydrostatic_curves.json".to_string();
    let strength =
        Strength::new_project(input_path, shiploads_file, frames_file, hydrostatic_curves).unwrap();
    strength.bending_moment();
    let vis = Visualisation::new(11.75, &strength);
    vis.show(DiagrammType::LightweightIntensity);
    vis.show(DiagrammType::DeadweightIntensity);
    vis.show(DiagrammType::DisplacementIntensity);
    vis.show(DiagrammType::BuoyancyIntensity);
    vis.show(DiagrammType::TotalShipload);
    vis.show(DiagrammType::ShareForce);
    vis.show(DiagrammType::ShareForceWithCorrection);
    vis.show(DiagrammType::BendingMoment);
    vis.show(DiagrammType::BendingMomentWithCorrection);
}
