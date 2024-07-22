use std::rc::Rc;

use log::info;

use super::lcg::LCG;
use crate::{
    core::round::Round,
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
///    lcb - абсцисса центра велечины,
///    displacement - объемное водоизмещение,
///    lcg - центр тяжести судна,
///    displacement_tonnage - весовое водоизмещение судна,
///    hydrostatic_curves - гидростатические кривые,
///    water_density - плотность воды.
pub(crate) struct ShipTrimming {
    lcb: Rc<LCB>,
    displacement: Rc<Displacement>,
    lcg: Rc<LCG>,
    d_t: Rc<DisplacementTonnage>,
    hydrostatic_curves: HydrostaticCurves,
}

impl ShipTrimming {
    ///
    /// Основной конструктор.
    pub fn new(
        lcb: Rc<LCB>,
        displacement: Rc<Displacement>,
        lcg: Rc<LCG>,
        d_t: Rc<DisplacementTonnage>,
        hydrostatic_curves: HydrostaticCurves,
    ) -> Self {
        ShipTrimming {
            lcb,
            displacement,
            lcg,
            d_t,
            hydrostatic_curves,
        }
    }

    ///
    /// Проверяет достигнута ли удифферентовка судна.
    /// Parameters:
    ///     displacement - водоизмещение судна при текущей схеме загрузки[м^3];
    ///     calc_disp - расчетное объемное водоизмещение судна [м^3].
    fn trim_achieved(&self, calc_disp: f64, lcb: f64, lbp: f64) -> Result<bool, String> {
        let displacement_tonnage = self.d_t.displacement_tonnage();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let lcg = self.lcg.lcg()?;
        Ok(self.error(calc_disp, displacement) <= 0.01
            && (lcg.abs() - lcb.abs()).abs() <= (0.001 * lbp))
    }
    ///
    /// Расчет процентного различия между двумя числами.
    fn error(&self, x_1: f64, x_2: f64) -> f64 {
        (((x_1.abs() - x_2.abs()).abs() / x_1.abs().min(x_2.abs())) * 100.0).my_round(2)
    }

    ///
    /// Выводит информацию о удифферентовки судна.
    fn solution_information(
        &self,
        i: u32,
        aft_draft: f64,
        nose_draft: f64,
        lbp: f64,
        calc_disp: f64,
    ) -> Result<(), String> {
        let displacement_tonnage = self.d_t.displacement_tonnage();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let lcg = self.lcg.lcg()?;
        let lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        info!("---------- Итерация № {i} ---------- ");
        info!("aft_draft = {} м, nose_draft = {} м", aft_draft, nose_draft);
        info!("mean_draft = {} м", 0.5 * (aft_draft + nose_draft));
        info!("lcg = {} м, lcb = {} м", lcg, lcb);
        info!("0.001 * lbp = {}", lbp * 0.001);
        info!("lcg - lcb = {}", (lcg.abs() - lcb.abs()).abs());
        if (lcg.abs() - lcb.abs()).abs() <= 0.001 * lbp {
            info!("|lcg - lcb| < lbp * 0.001 - условие удифферентовки судна для абсциссы центра велечины (lcb) выполняется.")
        } else {
            info!(
                "|lcg - lcb| > lbp * 0.001 - условие удифферентовки судна для абсциссы центра велечины (lcb) не выполняется."
            )
        }
        info!("Заданное водоизмещение: {displacement} м^3.");
        info!("Расчетное водоизмещение: {calc_disp} м^3.");
        if self.error(displacement, calc_disp) <= 0.01 {
            info!(
                "Процентная разница между расчетным и заданным водоизмещением судна = {} % - Условие удифферентовки судна для водоизмещения выполняется.",
                self.error(displacement, calc_disp)
            );
        } else {
            info!(
                "Процентная разница между расчетным и заданным водоизмещением судна = {} % - Условие удифферентовки судна для водоизмещения не выполняется.",
                self.error(displacement, calc_disp)
            );
        }
        Ok(())
    }
    ///
    /// Определяет коэффициент подобия треугольника.
    fn similarity_coefficient(&self, lcf: f64, lbp: f64) -> f64 {
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
        b / a
    }

    fn trimming(&self, lbp: f64, mean_draft: f64, lcf: f64) -> Result<(f64, f64), String> {
        let mut max_draft = self.hydrostatic_curves.max_draft();
        let mut min_draft = self.hydrostatic_curves.min_draft();
        let mut nose_draft = mean_draft;
        let mut aft_draft = mean_draft;
        let mut lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        let lcg = self.lcg.lcg()?;
        let mut j = 0;
        let similarity_coefficient = self.similarity_coefficient(lcf, lbp);
        while (lcg.abs() - lcb.abs()).abs() > 0.001 * lbp && j < 100 {
            if lcg < lcb {
                // Поворот ВЛ по часовой стрелки.
                if aft_draft > nose_draft {
                    max_draft = nose_draft;
                    nose_draft = (max_draft + min_draft) / 2.0;
                    aft_draft = ((mean_draft - nose_draft) / similarity_coefficient) + mean_draft;
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
                // Поворот ВЛ против часовой.
                if aft_draft > nose_draft {
                    min_draft = nose_draft;
                    nose_draft = (max_draft + min_draft) / 2.0;
                    aft_draft = ((mean_draft - nose_draft) / similarity_coefficient) + mean_draft;
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
            j += 1;
        }
        Ok((aft_draft, nose_draft))
    }

    ///
    /// Удифферентовка судна методом последовательных приближений.
    /// Возвращает осадку кормы и носа судна (aft_draft, nose_draft).
    pub fn trim(&self, ship_dimensions: &ShipDimensions) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.d_t.displacement_tonnage();
        if displacement_tonnage > self.hydrostatic_curves.max_displacement_tonnage() {
            return Err(format!("Весовое водоизмещение {displacement_tonnage} тонн превысило весовое водоизмещение судна в грузу."));
        }
        let mut mean_draft = self
            .hydrostatic_curves
            .mean_draft(displacement_tonnage)?
            .unwrap();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let lcf = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?
            .unwrap();
        let mut max_draft_d = self.hydrostatic_curves.max_draft();
        let mut min_draft_d = self.hydrostatic_curves.min_draft();
        let lbp = ship_dimensions.lbp();
        for i in 0..50 {
            let (aft_draft, nose_draft) = self.trimming(lbp, mean_draft, lcf)?;
            let calc_disp = self
                .displacement
                .displacement_by_drafts(aft_draft, nose_draft)?;
            self.solution_information(i, aft_draft, nose_draft, lbp, calc_disp)?;
            if (calc_disp - displacement).abs() <= 0.004 * displacement {
                return Ok((aft_draft, nose_draft));
            } else {
                if calc_disp < displacement {
                    min_draft_d = mean_draft;
                    mean_draft = (max_draft_d + min_draft_d) / 2.0;
                } else {
                    max_draft_d = mean_draft;
                    mean_draft = (max_draft_d + min_draft_d) / 2.0;
                }
            }
        }
        Err(
            "Удифферентовка судна не достигнута. Прeвышение максимального количества итераций."
                .to_string(),
        )
    }
}
