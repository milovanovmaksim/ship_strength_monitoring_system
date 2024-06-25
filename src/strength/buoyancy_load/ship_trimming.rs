use log::{debug, info};

use super::lcg::LCG;
use crate::{
    core::water_density::WaterDensity,
    strength::{
        bonjean_scale::lcb::LCB,
        displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage},
        hydrostatic_curves::{
            hydrostatic_curves::HydrostaticCurves, hydrostatic_typedata::HydrostaticTypeData,
        },
        ship::ship_dimensions::ShipDimensions,
    },
};

///
/// Удифферентовка судна на тихой воде.
/// Parameters:
pub(crate) struct ShipTrimming<'a> {
    lcb: LCB<'a>,
    displacement: Displacement<'a>,
    lcg: LCG<'a>,
    displacement_tonnage: DisplacementTonnage<'a>,
    hydrastatic_curves: HydrostaticCurves,
    ship_dimensions: ShipDimensions,
    water_density: WaterDensity,
}

impl<'a> ShipTrimming<'a> {
    ///
    /// Основной конструктор.
    pub fn new(
        lcb: LCB<'a>,
        displacement: Displacement<'a>,
        lcg: LCG<'a>,
        displacement_tonnage: DisplacementTonnage<'a>,
        hydrostatic_curves: HydrostaticCurves,
        ship_dimensions: ShipDimensions,
        water_density: WaterDensity,
    ) -> Self {
        ShipTrimming {
            lcb,
            displacement,
            lcg,
            displacement_tonnage,
            hydrastatic_curves: hydrostatic_curves,
            ship_dimensions,
            water_density,
        }
    }

    ///
    /// Проверяет достигнута ли удифферентовка судна.
    /// Parameters:
    ///     displacement_tonnage - весовое водоизмещение судна [т];
    ///     current_displacement - текущее расчетное объемное водоизмещение судна [м^3].
    fn trim_achieved(
        &self,
        displacement_tonnage: f64,
        current_displacement: f64,
        lcg: f64,
        lcb: f64,
    ) -> bool {
        (displacement_tonnage - self.water_density.water_density() * current_displacement).abs()
            <= 0.004 * displacement_tonnage
            && (lcg - lcb).abs() <= 0.001 * self.ship_dimensions.lbp()
    }

    ///
    /// Валидация средней осадки судна.
    /// Средняя осадка не должна превышать максимальную среднюю осадку судна.
    /// Parameters:
    ///     current_mean_draft - текущая расчетная средняя осадка судна.
    fn validate_mean_draft(&self, current_mean_draft: f64) -> Result<(), String> {
        let max_draft = self.hydrastatic_curves.max_draft();
        if current_mean_draft > max_draft {
            return Err(format!("Удифферентовка судна не достигнута из-за превышения максимальной средней осадки судна.\
                                Максимальная средняя осадка судна в грузу составляет {}. Расчетное значение: {}",
                                max_draft, current_mean_draft));
        };
        Ok(())
    }

    ///
    /// Удифферентовка судна методом последовательных приближений.
    /// Возвращает осадку кормы и носа судна (aft_draft, nose_draft).
    pub fn trim(&self) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        // TODO: Здесь нужно проверить произошло ли привышение водоизмещения судна в грузу.
        let mean_draft = self.hydrastatic_curves.mean_draft(displacement_tonnage)?
            .ok_or(format!("Удифферентовка судна не возможна,
            т.к в гидростатических кривых отсутствует средняя осадка для водоизмещения {displacement_tonnage} тонн."))?;
        let area_water_line = self
            .hydrastatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::WaterlineArea)?
            .ok_or(format!("Удифферентовка судна не возможна,
            т.к в гидростатических кривых отсутствует площадь ватер линии для осадки {mean_draft} м."))?;
        let lcg = self.lcg.lcg();
        let lbp = self.ship_dimensions.lbp();

        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let mut lcb = self
            .hydrastatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCB)?
            .ok_or(format!("Удифферентовка судна не возможна,
             т.к в гидростатических кривых отсутствует абсцисса центра велечины 'xc' для осадки {mean_draft} м."))?;
        let lmr = self
            .hydrastatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LMR)?
            .ok_or(format!(
                "Удифферентовка судна не возможна,
            т.к в гидростатических кривых отсутствует продольный метацентрический
            радиус 'R' для осадки {mean_draft} м."
            ))?;
        let lcf = self
            .hydrastatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?
            .ok_or(format!(
                "Удифферентовка судна не возможна, т.к в гидростатических кривых
            отсутствует абсцисса центра тяжести ватерлиниии xf для осадки {mean_draft} м."
            ))?;
        let nose_draft = mean_draft + ((lbp / 2.0) - lcf) * ((lcg - lcb) / lmr);
        let aft_draft = mean_draft - ((lbp / 2.0) + lcf) * ((lcg - lcb) / lmr);
        // TODO: првоерить вышли ли осадки за пределы допустимого диапазона.
        let mut calculated_displacement = self
            .displacement
            .displacement_by_drafts(aft_draft, nose_draft)?;
        lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        if self.trim_achieved(displacement_tonnage, calculated_displacement, lcg, lcb) {
            info!("Удифферентовка судна на тихой воде достигнута за 1 итерацию.");
            return Ok((aft_draft, nose_draft));
        };
        for i in 2..102 {
            let nose_draft = mean_draft
                + ((displacement - calculated_displacement) / area_water_line)
                + (lbp / 2.0 - lcf) * ((lcg - lcb) / lmr);

            let aft_draft = mean_draft
                + ((displacement - calculated_displacement) / area_water_line)
                - (lbp / 2.0 + lcf) * ((lcg - lcb) / lmr);
            // TODO: првоерить вышли ли осадки за пределы допустимого диапазона.
            let current_mean_draft = (aft_draft + nose_draft) * 0.5;
            calculated_displacement = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            let err = (lcg - lcb).abs();
            let displacement_error = (displacement_tonnage
                - self.water_density.water_density() * calculated_displacement)
                .abs();
            info!("displ_error = {displacement_error}");
            info!("aft_draft = {}, nose_draft = {}", aft_draft, nose_draft);
            info!("'lcg - lcb' = {}", err);
            info!("lcg = {}, lcb = {}", lcg, lcb);
            info!("ship_mean_draft = {current_mean_draft}");
            info!("displacement = {displacement}, calculated_displacement = {calculated_displacement}");
            if self.trim_achieved(displacement_tonnage, calculated_displacement, lcg, lcb) {
                info!("Удифферентовка судна на тихой воде достигнута за {i} итераций.");
                return Ok((aft_draft, nose_draft));
            };
            lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        }
        let err = (lcg - lcb).abs();
        let displacement_error = (displacement_tonnage
            - self.water_density.water_density() * calculated_displacement)
            .abs();
        debug!("displ_error = {displacement_error}");
        debug!("aft_draft = {}, nose_draft = {}", aft_draft, nose_draft);
        debug!("'lcg - lcb' = {}", err);
        debug!("lcg = {}, lcb = {}", lcg, lcb);
        debug!(
            "displacement = {displacement}, calculated_displacement = {calculated_displacement}"
        );
        Err("Удифферентовка судна не достигнута из-за превышения максимального количества итераций.".to_string())
    }
}
