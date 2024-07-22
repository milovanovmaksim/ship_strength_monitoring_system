use crate::{
    core::water_density::WaterDensity,
    strength::{
        bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames},
        deadweight::deadweight::Deadweight,
        displacement::displacement_tonnage::DisplacementTonnage,
    },
};

use super::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
    load::shiploads::Shiploads,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};

pub struct Strength<'a> {
    lightweight_intensity: &'a LightweightIntensity,
    dw_i: &'a DeadweightIntensity<'a>,
    ship_dimansions: &'a ShipDimensions,
}

impl<'a> Strength<'a> {
    pub fn new(
        lightweight_intensity: &'a LightweightIntensity,
        dw_i: &'a DeadweightIntensity<'a>,
        ship_dimansions: &'a ShipDimensions,
    ) -> Strength<'a> {
        Strength {
            lightweight_intensity,
            dw_i,
            ship_dimansions,
        }
    }

    pub fn new_project(
        input_path: String,
        shiploads_file: String,
        frames_file: String,
    ) -> Result<Self, String> {
        let lw = Lightweight::from_json_file("./input_data/input_data.json".to_string())?;
        let ship_dimensions =
            ShipDimensions::from_json_file("./input_data/input_data.json".to_string())?;
        let lw_i = LightweightIntensity::from_ship_input_data(&ship_dimensions, &lw);
        let shiploads = Shiploads::from_json_file(shiploads_file)?;
        let dw_i = DeadweightIntensity::new(&shiploads);
        let disp_i = DisplacementIntensity::new(&dw_i, &lw_i);
        let water_density = WaterDensity::new(1.025);
        let frames = Frames::from_json_file(frames_file)?;
        let bonjean_scale = BonjeanScale::new(frames, ship_dimensions.clone());
        let dw = Deadweight::new(&shiploads);
        let d_t = DisplacementTonnage::new(lw, &dw);

        todo!()
    }

    pub fn lightweight_intensity(&self) -> &SpatiumFunctions {
        self.lightweight_intensity.lightweight_intensity()
    }

    pub fn dedweight_intensity(&self) -> &SpatiumFunctions {
        self.dw_i.deadweight_intensity(self.ship_dimansions)
    }
}
