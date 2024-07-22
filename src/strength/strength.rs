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
        bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB}, buoyancy_intensity::{
            buoyancy_intensity::BuoyancyIntensity, lcg::LCG, ship_trimming::ShipTrimming,
        }, deadweight::deadweight::Deadweight, displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage}, hydrostatic_curves::hydrostatic_curves::HydrostaticCurves, internal_forces::{bending_moment::BendingMoment, share_force::ShareForce}, load::total_shipload::TotalShipload
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
        hydrostatic_curves: String,
    ) -> Result<Self, String> {
        let lw = Lightweight::from_json_file(input_path.clone())?;
        let ship_dimensions = ShipDimensions::from_json_file(input_path.clone())?;
        let lw_i = Rc::new(LightweightIntensity::from_ship_input_data(
            &ship_dimensions,
            lw,
        ));
        let shiploads = Rc::new(Shiploads::from_json_file(shiploads_file)?);
        let dw_i = Rc::new(DeadweightIntensity::new(
            shiploads.clone(),
            ship_dimensions.clone(),
        ));
        let disp_i = Rc::new(DisplacementIntensity::new(
            dw_i.clone(),
            lw_i.clone(),
            ship_dimensions.clone(),
        ));
        let dw = Rc::new(Deadweight::new(shiploads.clone()));
        let d_t = Rc::new(DisplacementTonnage::new(lw, dw.clone()));
        let water_density = WaterDensity::from_json_file(input_path.clone())?;
        let frames = Frames::from_json_file(frames_file)?;
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions.clone()));
        let disp = Rc::new(Displacement::new(
            bonjean_scale.clone(),
            ship_dimensions.clone(),
            water_density,
        ));
        let lcb = Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions.clone()));
        let lcg = Rc::new(LCG::new(disp_i.clone(), ship_dimensions.clone()));
        let hydrostatic_curves = HydrostaticCurves::from_json_file(hydrostatic_curves)?;
        let ship_trimming = ShipTrimming::new(
            lcb.clone(),
            disp.clone(),
            lcg.clone(),
            d_t.clone(),
            hydrostatic_curves,
        );
        let b_i = Rc::new(BuoyancyIntensity::new(
            ship_trimming,
            bonjean_scale.clone(),
            water_density,
        ));
        let total_shipload = TotalShipload::new(disp_i.clone(), b_i.clone());
        let share_force = ShareForce::new(total_shipload);
        let bending_moment = BendingMoment::new(share_force)
        todo!()
    }
}
