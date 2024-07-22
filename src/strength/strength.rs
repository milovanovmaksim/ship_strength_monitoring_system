use super::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
    load::shiploads::Shiploads,
    ship::ship_dimensions::ShipDimensions,
};
use crate::{
    core::water_density::WaterDensity,
    strength::{
        bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames},
        deadweight::deadweight::Deadweight,
        displacement::displacement_tonnage::DisplacementTonnage,
    },
};
use std::rc::Rc;

pub struct Strength {}

impl Strength {
    pub fn new() -> Strength {
        todo!()
    }

    pub fn new_project(
        input_path: String,
        shiploads_file: String,
        frames_file: String,
    ) -> Result<Self, String> {
        let lw = Lightweight::from_json_file(input_path.clone())?;
        let ship_dimensions = ShipDimensions::from_json_file(input_path.clone())?;
        let lw_i = Rc::new(LightweightIntensity::from_ship_input_data(
            &ship_dimensions,
            lw,
        ));
        let shiploads = Rc::new(Shiploads::from_json_file(shiploads_file)?);
        let dw_i = Rc::new(DeadweightIntensity::new(shiploads.clone()));
        let disp_i = Rc::new(DisplacementIntensity::new(dw_i.clone(), lw_i.clone()));
        let water_density = WaterDensity::from_json_file(input_path.clone());
        let frames = Frames::from_json_file(frames_file)?;
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions.clone());
        let dw = Rc::new(Deadweight::new(shiploads.clone()));
        let d_t = Rc::new(DisplacementTonnage::new(lw, dw.clone()));

        todo!()
    }
}
