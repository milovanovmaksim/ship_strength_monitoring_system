use super::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
    load::shiploads::Shiploads,
    ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
};
use crate::{
    core::water_density::WaterDensity,
    strength::{
        bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
        buoyancy_intensity::{
            buoyancy_intensity::BuoyancyIntensity, lcg::LCG, ship_trimming::ShipTrimming,
        },
        deadweight::deadweight::Deadweight,
        displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage},
        hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
        internal_forces::{bending_moment::BendingMoment, share_force::ShareForce},
        load::total_shipload::TotalShipload,
    },
};
use std::rc::Rc;

///
/// Структура для расчета прочности судна.
/// Входные параметры:
///    lw - lightweight,
pub(crate) struct Strength {
    lw: Lightweight,
    lw_i: Rc<LightweightIntensity>,
    dw: Rc<Deadweight>,
    dw_i: Rc<DeadweightIntensity>,
    disp: Rc<Displacement>,
    disp_i: Rc<DisplacementIntensity>,
    d_t: Rc<DisplacementTonnage>,
    lcb: Rc<LCB>,
    lcg: Rc<LCG>,
    b_i: Rc<BuoyancyIntensity>,
    total_shipload: Rc<TotalShipload>,
    share_force: Rc<ShareForce>,
    bending_moment: Rc<BendingMoment>,
    water_density: WaterDensity,
    ship_dimensions: ShipDimensions,
}

impl Strength {
    ///
    /// Основной конструктор.
    pub fn new(
        lw: Lightweight,
        lw_i: Rc<LightweightIntensity>,
        dw: Rc<Deadweight>,
        dw_i: Rc<DeadweightIntensity>,
        disp: Rc<Displacement>,
        disp_i: Rc<DisplacementIntensity>,
        d_t: Rc<DisplacementTonnage>,
        lcb: Rc<LCB>,
        lcg: Rc<LCG>,
        b_i: Rc<BuoyancyIntensity>,
        total_shipload: Rc<TotalShipload>,
        share_force: Rc<ShareForce>,
        bending_moment: Rc<BendingMoment>,
        water_density: WaterDensity,
        ship_dimensions: ShipDimensions,
    ) -> Strength {
        Strength {
            lw,
            lw_i,
            dw,
            dw_i,
            disp,
            disp_i,
            d_t,
            lcb,
            lcg,
            b_i,
            total_shipload,
            share_force,
            bending_moment,
            water_density,
            ship_dimensions,
        }
    }

    ///
    /// Вспомогательный конструктор.
    /// Входные параметры:
    ///     input_path - путь к json файлу, содержащему основные данные о судне,
    ///     shiploads_file - путь к json файлу, содержащему нагрузки, действующие на судно,
    ///     frames_file - путь к json файлу, содержащему масштаб Бонжана,
    ///     hydrostatic_curves - пусть к файлу, содержащему гидростатические кривые судна.
    pub fn new_project(
        input_path: String,
        shiploads_file: String,
        frames_file: String,
        hydrostatic_curves: String,
    ) -> Result<Self, String> {
        let lw = Lightweight::from_json_file(input_path.clone())?;
        let ship_dimensions = ShipDimensions::from_json_file(input_path.clone())?;
        let lw_i = Rc::new(LightweightIntensity::from_ship_input_data(
            ship_dimensions.clone(),
            lw,
        ));
        let shiploads = Rc::new(Shiploads::from_json_file(shiploads_file)?);
        let dw_i = Rc::new(DeadweightIntensity::new(shiploads.clone(), ship_dimensions));
        let disp_i = Rc::new(DisplacementIntensity::new(
            dw_i.clone(),
            lw_i.clone(),
            ship_dimensions,
        ));
        let dw = Rc::new(Deadweight::new(shiploads));
        let d_t = Rc::new(DisplacementTonnage::new(lw, dw.clone()));
        let water_density = WaterDensity::from_json_file(input_path.clone())?;
        let frames = Frames::from_json_file(frames_file)?;
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let disp = Rc::new(Displacement::new(
            bonjean_scale.clone(),
            ship_dimensions,
            water_density,
        ));
        let lcb = Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions));
        let lcg = Rc::new(LCG::new(disp_i.clone(), ship_dimensions));
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
            bonjean_scale,
            water_density,
        ));
        let total_shipload = Rc::new(TotalShipload::new(disp_i.clone(), b_i.clone()));
        let share_force = Rc::new(ShareForce::new(total_shipload.clone()));
        let bending_moment = Rc::new(BendingMoment::new(share_force.clone()));
        Ok(Strength::new(
            lw,
            lw_i,
            dw,
            dw_i,
            disp,
            disp_i,
            d_t,
            lcb,
            lcg,
            b_i,
            total_shipload,
            share_force,
            bending_moment,
            water_density,
            ship_dimensions,
        ))
    }

    pub fn lightweight_intensity(&self) -> &SpatiumFunctions {
        self.lw_i.lightweight_intensity()
    }
}
