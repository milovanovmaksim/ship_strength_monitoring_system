use log::{debug, info};

use super::lcg::LCG;
use crate::{
    core::{round::Round, water_density::WaterDensity},
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
    hydrostatic_curves: HydrostaticCurves,
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
            hydrostatic_curves,
            ship_dimensions,
            water_density,
        }
    }

    ///
    /// Проверяет достигнута ли удифферентовка судна.
    /// Parameters:
    ///     displacement - водоизмещение судна при текущей схеме загрузки[м^3];
    ///     calc_disp - расчетное объемное водоизмещение судна [м^3].
    fn trim_achieved(&self, displacement: f64, calc_disp: f64, lcg: f64, lcb: f64) -> bool {
        self.error(calc_disp, displacement) <= 2.0
            && (lcg.abs() - lcb.abs()).abs() <= (0.001 * self.ship_dimensions.lbp())
    }

    fn error(&self, x_1: f64, x_2: f64) -> f64 {
        (((x_1.abs() - x_2.abs()).abs() / x_1.abs().min(x_2.abs())) * 100.0).my_round(2)
    }

    ///
    /// Возвращает валидные осадки кормы и носа судна.
    /// Осадки не должны выходить за диапазон осадки судна указанный в гидростатических кривых.
    fn drafts(&self, mut aft_draft: f64, mut nose_draft: f64) -> (f64, f64) {
        if aft_draft > self.hydrostatic_curves.max_draft() {
            aft_draft = self.hydrostatic_curves.max_draft();
        } else if aft_draft < self.hydrostatic_curves.min_draft() {
            aft_draft = self.hydrostatic_curves.min_draft();
        }
        if nose_draft > self.hydrostatic_curves.max_draft() {
            nose_draft = self.hydrostatic_curves.max_draft();
        } else if nose_draft < self.hydrostatic_curves.min_draft() {
            nose_draft = self.hydrostatic_curves.min_draft();
        }

        (aft_draft, nose_draft)
    }

    fn solution_information(
        &self,
        aft_draft: f64,
        nose_draft: f64,
        lcg: f64,
        lcb: f64,
        displacement: f64,
        calc_disp: f64,
    ) {
        info!("aft_draft = {} м, nose_draft = {} м", aft_draft, nose_draft);
        info!("mean_draft = {} м", 0.5 * (aft_draft + nose_draft));
        info!("lcg = {} м, lcb = {} м", lcg, lcb);
        info!("0.001 * lbp = {}", self.ship_dimensions.lbp() * 0.001);
        info!("lcg - lcb = {}", (lcg.abs() - lcb.abs()).abs());
        if (lcg.abs() - lcb.abs()).abs() <= 0.001 * self.ship_dimensions.lbp() {
            info!("|lcg - lcb| < lbp * 0.001 Ok")
        } else {
            info!("|lcg - lcb| > lbp * 0.001 Bad")
        }
        info!("displacement = {displacement} м^3, calc_displacement = {calc_disp} м^3");
        info!(
            "Разница между расчетным и заданным водоизмещением = {} %",
            self.error(displacement, calc_disp)
        );
        info!("---------------------------------------\n");
    }

    pub fn trim(&self) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        if displacement_tonnage > self.hydrostatic_curves.max_displacement_tonnage() {
            return Err(format!("Весовое водоизмещение {displacement_tonnage} тонн превысило весовое водоизмещение судна в грузу."));
        }
        let mut mean_draft = self
            .hydrostatic_curves
            .mean_draft(displacement_tonnage)?
            .unwrap();
        let lcg = self.lcg.lcg();
        let lbp = self.ship_dimensions.lbp();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let lcf = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?
            .unwrap();
        let mut max_draft_d = self.hydrostatic_curves.max_draft();
        let mut min_draft_d = self.hydrostatic_curves.min_draft();
        let (a, b) = {
            if lcf < 0.0 {
                let a = lbp / 2.0 - lcf.abs();
                let b = lbp / 2.0 + lcf.abs();
                (a, b)
            } else {
                let a = lbp / 2.0 + lcf.abs();
                let b = lbp / 2.0 - lcf.abs();
                (a, b)
            }
        };
        let similarity_coefficient = b / a;
        info!("{similarity_coefficient}");
        for _ in 0..10 {
            let mut i = 0;
            let mut max_draft = self.hydrostatic_curves.max_draft();
            let mut min_draft = self.hydrostatic_curves.min_draft();
            let mut nose_draft = mean_draft;
            let mut aft_draft = mean_draft;
            let mut lcb = self.lcb.lcb(aft_draft, nose_draft)?;
            let mut calc_disp = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            self.solution_information(aft_draft, nose_draft, lcg, lcb, displacement, calc_disp);
            if self.trim_achieved(displacement, calc_disp, lcg, lcb) {
                info!("Удифферентовка судна на тихой воде достигнута.");
                return Ok((aft_draft.my_round(2), nose_draft.my_round(2)));
            }
            while (lcg.abs() - lcb.abs()).abs() > 0.001 * lbp && i <= 10 {
                if lcg < lcb {
                    if aft_draft > nose_draft {
                        max_draft = nose_draft;
                        nose_draft = (max_draft + min_draft) / 2.0;
                        aft_draft =
                            ((mean_draft - nose_draft) / similarity_coefficient) + mean_draft;
                    } else if aft_draft < nose_draft {
                        max_draft = nose_draft;
                        nose_draft = (min_draft + max_draft) / 2.0;
                        aft_draft = mean_draft - (nose_draft - mean_draft) / similarity_coefficient;
                    } else {
                        min_draft = mean_draft;
                        aft_draft = (min_draft + max_draft) / 2.0;
                        nose_draft = mean_draft - (aft_draft - mean_draft) * similarity_coefficient;
                    }
                } else if lcg > lcb {
                    if aft_draft > nose_draft {
                        min_draft = nose_draft;
                        nose_draft = (max_draft + min_draft) / 2.0;
                        aft_draft =
                            ((mean_draft - nose_draft) / similarity_coefficient) + mean_draft;
                    } else if aft_draft < nose_draft {
                        min_draft = nose_draft;
                        nose_draft = (max_draft + min_draft) / 2.0;
                        aft_draft = mean_draft - (nose_draft - mean_draft) / similarity_coefficient;
                    } else {
                        min_draft = mean_draft;
                        nose_draft = (max_draft + min_draft) / 2.0;
                        aft_draft = mean_draft - (nose_draft - mean_draft) / similarity_coefficient;
                    }
                }
                lcb = self.lcb.lcb(aft_draft, nose_draft)?;
                calc_disp = self
                    .displacement
                    .displacement_by_drafts(aft_draft, nose_draft)?;
                info!("---------------Итерация №{i}------------------");
                self.solution_information(aft_draft, nose_draft, lcg, lcb, displacement, calc_disp);
                i += 1;
                if self.trim_achieved(displacement, calc_disp, lcg, lcb) {
                    info!("Удифферентовка судна на тихой воде достигнута.");
                    return Ok((aft_draft.my_round(2), nose_draft.my_round(2)));
                }
            }
            if calc_disp < displacement {
                min_draft_d = mean_draft;
                mean_draft = (max_draft_d + min_draft_d) / 2.0;
            } else {
                max_draft_d = mean_draft;
                mean_draft = (max_draft_d + min_draft_d) / 2.0;
            }
            info!("Новое значение средней осадки: {mean_draft} м");
        }
        Err("Удифферентовка судна не возможна.".to_string())
    }

    ///
    /// Удифферентовка судна методом последовательных приближений.
    /// Возвращает осадку кормы и носа судна (aft_draft, nose_draft).
    pub fn trim_2(&self) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        if displacement_tonnage > self.hydrostatic_curves.max_displacement_tonnage() {
            return Err(format!("Весовое водоизмещение {displacement_tonnage} тонн превысило весовое водоизмещение судна в грузу."));
        }
        let mean_draft = self
            .hydrostatic_curves
            .mean_draft(displacement_tonnage)?
            .unwrap();
        info!("mean_draft = {mean_draft}");
        let area_water_line = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::WaterlineArea)?
            .unwrap();
        let lcg = self.lcg.lcg();
        let lbp = self.ship_dimensions.lbp();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let mut lcb = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCB)?
            .unwrap();
        let lmr = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LMR)?
            .unwrap();
        let lcf = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?
            .unwrap();
        let mut nose_draft = mean_draft + ((lbp / 2.0) - lcf) * ((lcg - lcb) / lmr);
        let mut aft_draft = mean_draft - ((lbp / 2.0) + lcf) * ((lcg - lcb) / lmr);
        (aft_draft, nose_draft) = self.drafts(aft_draft, nose_draft);
        let mut calculated_displacement = self
            .displacement
            .displacement_by_drafts(aft_draft, nose_draft)?;
        lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        self.solution_information(
            aft_draft,
            nose_draft,
            lcg,
            lcb,
            displacement,
            calculated_displacement,
        );
        if self.trim_achieved(displacement, calculated_displacement, lcg, lcb) {
            info!("Удифферентовка судна на тихой воде достигнута за 1 итерацию.");
            return Ok((aft_draft.my_round(2), nose_draft.my_round(2)));
        };
        for i in 2..20 {
            let mut nose_draft = mean_draft
                + ((displacement - calculated_displacement) / area_water_line)
                + (lbp / 2.0 - lcf) * ((lcg - lcb) / lmr);

            let mut aft_draft = mean_draft
                + ((displacement - calculated_displacement) / area_water_line)
                - (lbp / 2.0 + lcf) * ((lcg - lcb) / lmr);
            (nose_draft, aft_draft) = self.drafts(aft_draft, nose_draft);
            calculated_displacement = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            lcb = self.lcb.lcb(aft_draft, nose_draft)?;
            self.solution_information(
                aft_draft,
                nose_draft,
                lcg,
                lcb,
                displacement,
                calculated_displacement,
            );
            if self.trim_achieved(displacement, calculated_displacement, lcg, lcb) {
                info!("Удифферентовка судна на тихой воде достигнута за {i} итераций.");
                return Ok((aft_draft.my_round(2), nose_draft.my_round(2)));
            };
        }
        Err("Удифферентовка судна не достигнута из-за превышения максимального количества итераций.".to_string())
    }
}
