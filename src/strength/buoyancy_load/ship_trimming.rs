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
    ///     current_displacement - расчетное объемное водоизмещение судна [м^3].
    fn trim_achieved(
        &self,
        displacement: f64,
        calculated_displacement: f64,
        lcg: f64,
        lcb: f64,
    ) -> bool {
        self.trim_error(displacement, calculated_displacement) <= 5.0
            && (lcg - lcb).abs().my_round(2) <= (0.001 * self.ship_dimensions.lbp()).my_round(2)
    }

    fn trim_error(&self, x_1: f64, x_2: f64) -> f64 {
        (((x_1 - x_2).abs() / x_1.abs().min(x_2.abs())) * 100.0).my_round(2)
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
        calculated_displacement: f64,
    ) {
        info!("aft_draft = {} м, nose_draft = {} м", aft_draft, nose_draft);
        info!("mean_draft = {}", 0.5 * (aft_draft + nose_draft));
        info!("lcg = {}, lcb = {}", lcg, lcb);
        info!("0.001 * lbp = {}", self.ship_dimensions.lbp() * 0.001);
        info!("'lcg - lcb' = {}", (lcg.abs() - lcb.abs()).abs());
        info!("displacement = {displacement}, calculated_displacement = {calculated_displacement}");
        info!(
            "error_displacement = {} %",
            self.trim_error(displacement, calculated_displacement)
        );
        info!("---------------------------------------");
    }

    pub fn trim_2(&self) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        if displacement_tonnage > self.hydrostatic_curves.max_displacement_tonnage() {
            return Err(format!("Весовое водоизмещение {displacement_tonnage} тонн превысило весовое водоизмещение судна в грузу."));
        }
        let mut mean_draft = self
            .hydrostatic_curves
            .mean_draft(displacement_tonnage)?
            .unwrap();
        info!("mean_draft = {mean_draft}");
        let lcg = self.lcg.lcg();
        let lbp = self.ship_dimensions.lbp();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let lcf = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?
            .unwrap();
        info!("lcf = {lcf}");
        let max_draft = self.hydrostatic_curves.max_draft();
        let min_draft = self.hydrostatic_curves.min_draft();
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
        info!("a = {}, b = {}", a, b);
        let similarity_coefficient = a / b;
        let mut nose_draft = mean_draft;
        let mut aft_draft = mean_draft;
        let calculated_displacement = self
            .displacement
            .displacement_by_drafts(aft_draft, nose_draft)?;
        let mut lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        self.solution_information(
            aft_draft,
            nose_draft,
            lcg,
            lcb,
            displacement,
            calculated_displacement,
        );
        if self.trim_achieved(displacement, calculated_displacement, lcg, lcb) {
            info!("Удифферентовка судна на тихой воде достигнута.");
            return Ok((aft_draft, nose_draft));
        }

        for _ in 0..20 {
            if lcg.abs() > lcb.abs() {
                info!("ВЛ поворачиваем по часовой. LCB сместится влево.");
                if aft_draft > nose_draft {
                    nose_draft = (nose_draft + min_draft) / 2.0;
                    aft_draft = ((mean_draft - nose_draft) / similarity_coefficient) + mean_draft;
                    lcb = self.lcb.lcb(aft_draft, nose_draft)?;
                } else if aft_draft < nose_draft {
                    todo!()
                } else {
                    todo!()
                }
            } else if lcb.abs() > lcg.abs() {
                info!("ВЛ поворачиваем против часовой. LCB сместится вправо.");
                if aft_draft > nose_draft {
                    nose_draft = (nose_draft + mean_draft) / 2.0;
                    aft_draft = ((mean_draft - nose_draft) / similarity_coefficient) + mean_draft;
                } else if aft_draft < nose_draft {
                    todo!()
                } else {
                    todo!()
                }
            }
        }
        todo!()
    }

    ///
    /// Удифферентовка судна методом последовательных приближений.
    /// Возвращает осадку кормы и носа судна (aft_draft, nose_draft).
    pub fn trim(&self) -> Result<(f64, f64), String> {
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
