use super::{
    deadweight::deadweight_intensity::DeadweightIntensity,
    displacement::displacement_intensity::DisplacementIntensity,
    lightweight::{lightweight::Lightweight, lightweight_intensity::LightweightIntensity},
    load::shiploads::Shiploads,
    ship::{ship_dimensions::ShipDimensions, spatium_functions::SpatiumFunctions},
};
use crate::{
    core::water_density::WaterDensity,
    strength::{
        bonjean_scale::{bonjean_scale::BonjeanScale, frames::Frames, lcb::LCB},
        buoyancy_intensity::{buoyancy_intensity::BuoyancyIntensity, draft::Draft, lcg::LCG},
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
///    lw - масса пустого корпуса судна  [т],
///    lw_i - интенсивность массы пустого корпуса судна по длине (LightweightIntensity)[т/м],
///    dw - дедвейт - общая масса груза на судне. Включает в себя топливо, смазочные материалы,
///    запасы воды, экипаж, пищевые припасы, пассажиров с их вещами и перевозимые товары [т].
///    dw_i - интенсивность дедвейта по длине судна [т/м],
///    disp - объемное водоизмещение судна [м^3],
///    disp_i - интенсивность весового водоизмещения судна по его длине [т/м],
///    d_t - весовое водоизмещение судна [т],
///    lcb_ - aбсцисса центра велечины (центр тяжести погруженного объема судна) [м],
///    lcg_ - абсцисса центра тяжести судна [м],
///    b_i - интенсивность сил поддержания судна [т/м],
///    total_shipload_ - интенсивность суммарной нагрузки, действующей на корпус судна [т/м],
///    share_force_ - эпюра перерезывающей силы [т],
///    bending_moment_ - эпюра изгибающего момента [т*м],
///    water_density - плотность воды, [т/м^3],
///    ship_dimensions - размерения судна,
///    draft - осадка судна при текущей схеме загрузки.
pub struct Strength {
    lw: Lightweight,
    lw_i: LightweightIntensity,
    dw: Deadweight,
    dw_i: DeadweightIntensity,
    disp: Rc<Displacement>,
    disp_i: DisplacementIntensity,
    d_t: DisplacementTonnage,
    lcb_: Rc<LCB>,
    lcg_: LCG,
    b_i: BuoyancyIntensity,
    total_shipload_: TotalShipload,
    share_force_: ShareForce,
    bending_moment_: BendingMoment,
    water_density: WaterDensity,
    ship_dimensions: ShipDimensions,
    draft_: Draft,
}

impl Strength {
    ///
    /// Основной конструктор.
    pub fn new(
        lw: Lightweight,
        lw_i: LightweightIntensity,
        dw: Deadweight,
        dw_i: DeadweightIntensity,
        disp: Rc<Displacement>,
        disp_i: DisplacementIntensity,
        d_t: DisplacementTonnage,
        lcb_: Rc<LCB>,
        lcg_: LCG,
        b_i: BuoyancyIntensity,
        total_shipload_: TotalShipload,
        share_force_: ShareForce,
        bending_moment_: BendingMoment,
        water_density: WaterDensity,
        ship_dimensions: ShipDimensions,
        draft_: Draft,
    ) -> Strength {
        Strength {
            lw,
            lw_i,
            dw,
            dw_i,
            disp,
            disp_i,
            d_t,
            lcb_,
            lcg_,
            b_i,
            total_shipload_,
            share_force_,
            bending_moment_,
            water_density,
            ship_dimensions,
            draft_,
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
        let lw_i = LightweightIntensity::from_ship_input_data(ship_dimensions.clone(), lw);
        let shiploads = Shiploads::from_json_file(shiploads_file)?;
        let dw_i = DeadweightIntensity::builder(&shiploads, ship_dimensions).build();
        let disp_i = DisplacementIntensity::from_dw_i_and_lw_i(&dw_i, &lw_i)?;
        let dw = Deadweight::from_shiplods(&shiploads);
        let d_t = DisplacementTonnage::new(lw, dw);
        let water_density = WaterDensity::from_json_file(input_path.clone())?;
        let frames = Frames::from_json_file(frames_file)?;
        let bonjean_scale = Rc::new(BonjeanScale::new(frames, ship_dimensions));
        let disp = Rc::new(Displacement::new(
            bonjean_scale.clone(),
            ship_dimensions,
            water_density,
        ));
        let lcb = Rc::new(LCB::new(bonjean_scale.clone(), ship_dimensions));
        let lcg = LCG::from_disp_i(&disp_i);
        let hydrostatic_curves = HydrostaticCurves::from_json_file(hydrostatic_curves)?;
        let draft = Draft::new(lcb.clone(), disp.clone(), lcg, d_t, hydrostatic_curves);
        let b_i =
            BuoyancyIntensity::constructor(ship_dimensions, &draft, &bonjean_scale, water_density)?;
        let total_shipload = TotalShipload::from_disp_i_and_b_i(&disp_i, &b_i)?;
        let share_force =
            ShareForce::from_total_ship_load(&total_shipload).with_correction(ship_dimensions);
        let bending_moment =
            BendingMoment::from_share_force(&share_force).with_correction(ship_dimensions);
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
            draft,
        ))
    }

    ///
    /// Интенсивность массы пустого корпуса судна по его длине. Размерность: [т/м].
    pub fn lightweight_intensity(&self) -> &SpatiumFunctions {
        self.lw_i.lightweight_intensity()
    }

    ///
    /// Масса пустого корпуса судна. Размерность: [т]
    pub fn lightweight(&self) -> f64 {
        self.lw.lightweight()
    }

    ///
    /// Общая масса груза на судне. Размерность: [т].
    pub fn deadweight(&self) -> f64 {
        self.dw.deadweight()
    }

    ///
    /// Интенсивность дедвейта по длине судна. Размерность: [т/м].
    pub fn deadweight_intensity(&self) -> &SpatiumFunctions {
        self.dw_i.deadweight_intensity()
    }

    ///
    /// Объемное водоизмещение судна. Размерность: [м^3].
    pub fn displacement(&self) -> Result<f64, String> {
        let (aft_draft, nose_draft) = self.draft_.draft(self.ship_dimensions)?;
        self.disp.displacement_by_drafts(aft_draft, nose_draft)
    }

    ///
    /// Интенсивность весового водоизмещения судна по его длине. Размерность: [т/м].
    pub fn displacement_intensity(&self) -> &SpatiumFunctions {
        self.disp_i.displacement_intensity()
    }

    ///
    /// Весовое водоизмещение судна. Размерность: [т].
    pub fn displacemnt_tonnage(&self) -> f64 {
        self.d_t.displacement_tonnage()
    }

    ///
    /// Осадка судна при текущей схеме загрузки.
    /// Возвращает осадку кормы и носа судна (aft_draft [м], nose_nose [м]).
    pub fn draft(&self) -> Result<(f64, f64), String> {
        self.draft_.draft(self.ship_dimensions)
    }

    ///
    /// Абсцисса центра велечины (центр тяжести погруженного объема судна). Размерность: [м].
    /// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
    pub fn lcb(&self) -> Result<f64, String> {
        let (aft_draft, nose_draft) = self.draft_.draft(self.ship_dimensions)?;
        self.lcb_.lcb(aft_draft, nose_draft)
    }

    ///
    /// Абсцисса центра тяжести судна. Размерность: [м].
    /// Отсчитывается от мидель шпангоута. Имеет положительный знак от мидель шпангоута в нос судна.
    pub fn lcg(&self) -> f64 {
        self.lcg_.lcg()
    }

    ///
    /// Интенсивность сил поддержания судна. Размерность: [т/м].
    pub fn buoyancy_intensity(&self) -> &SpatiumFunctions {
        self.b_i.buoyancy_intensity()
    }

    ///
    /// Интенсивность суммарной нагрузки, действующей на корпус судна. Размерность [т/м].
    pub fn total_shipload(&self) -> &SpatiumFunctions {
        self.total_shipload_.total_shipload()
    }

    ///
    /// Эпюра перерезывающих сил. Размерность: [т].
    pub fn share_force(&self) -> &SpatiumFunctions {
        self.share_force_.share_force()
    }
    ///
    /// Эпюра перерезывающих сил c поправкой, т.е перерезывающая
    /// сила в нсосовом и кормовом шпангоутах равна нулю. Размерность: [т].
    pub fn share_force_with_correction(&self) -> Option<&SpatiumFunctions> {
        self.share_force_.share_force_with_correction()
    }

    ///
    /// Эпюра изгибающих моментов. Размерность: [т * м].
    pub fn bending_moment(&self) -> &SpatiumFunctions {
        self.bending_moment_.bending_momant()
    }

    ///
    /// Эпюра изгибающих моментов c поправкой, т.е изгибающий момент
    /// в нсосовом и кормовом шпангоутах равен нулю. Размерность: [т].
    pub fn bending_moment_with_correction(&self) -> Option<&SpatiumFunctions> {
        self.bending_moment_.bending_moment_with_correction()
    }
}
