use std::ops::Add;

use log::info;

use super::ship_trimming::ShipTrimming;
use crate::{
    core::round::Round,
    strength::{
        bonjean_scale::lcb::LCB,
        buoyancy_intensity::lcg::LCG,
        displacement::{displacement::Displacement, displacement_tonnage::DisplacementTonnage},
        hydrostatic_curves::hydrostatic_curves::HydrostaticCurves,
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
pub(crate) struct Draft<'a> {
    displacement: Displacement<'a>,
    ship_trimming: ShipTrimming<'a>,
    displacement_tonnage: DisplacementTonnage<'a>,
    hydrostatic_curves: &'a HydrostaticCurves,
    lcb: &'a LCB<'a>,
    lcg: &'a LCG<'a>,
}

impl<'a> Draft<'a> {
    ///
    /// Основной конструктор.
    pub fn new(
        displacement: Displacement<'a>,
        ship_trimming: ShipTrimming<'a>,
        displacement_tonnage: DisplacementTonnage<'a>,
        hydrostatic_curves: &'a HydrostaticCurves,
        lcb: &'a LCB<'a>,
        lcg: &'a LCG<'a>,
    ) -> Self {
        Draft {
            displacement,
            ship_trimming,
            displacement_tonnage,
            hydrostatic_curves,
            lcb,
            lcg,
        }
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
        lcg: f64,
        lcb: f64,
        lbp: f64,
        displacement: f64,
        calc_disp: f64,
    ) {
        info!("-------- Итерация № {i} --------");
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
        if (calc_disp - displacement).abs() <= 0.004 * displacement {
            info!("|calculate_displacement - diplacement| < 0.004 *displacement - Условие удифферентовки судна для водоизмещения выполняется");
        } else {
            info!("|calculate_displacement - diplacement| > 0.004 *displacement - Условие удифферентовки судна для водоизмещения не выполняется");
        }
    }

    ///
    /// Возвращает осадки кормы и носа судна (aft_draft, nose_draft), удовлетваряющие условию удифферентовки судна.
    pub fn draft(&self, ship_dimensions: &ShipDimensions) -> Result<(f64, f64), String> {
        let displacement_tonnage = self.displacement_tonnage.displacement_tonnage();
        if displacement_tonnage > self.hydrostatic_curves.max_displacement_tonnage() {
            return Err(format!("Весовое водоизмещение {displacement_tonnage} тонн превысило весовое водоизмещение судна в грузу."));
        }
        let mut mean_draft = self
            .hydrostatic_curves
            .mean_draft(displacement_tonnage)?
            .unwrap();
        let displacement = self.displacement.displacement_by_mass(displacement_tonnage);
        let mut max_draft = self.hydrostatic_curves.max_draft();
        let mut min_draft = self.hydrostatic_curves.min_draft();
        let lbp = ship_dimensions.lbp();
        let lcg = self.lcg.lcg()?;
        for i in 0..50 {
            if let Some((aft_draft, nose_draft)) =
                self.ship_trimming.trimming(mean_draft, ship_dimensions)?
            {
                let calculate_displacement = self
                    .displacement
                    .displacement_by_drafts(aft_draft, nose_draft)?;
                let lcb = self.lcb.lcb(aft_draft, nose_draft)?;
                self.solution_information(
                    i,
                    aft_draft,
                    nose_draft,
                    lcg,
                    lcb,
                    lbp,
                    displacement,
                    calculate_displacement,
                );
                if (calculate_displacement - displacement).abs() <= 0.004 * displacement {
                    return Ok((aft_draft, nose_draft));
                }
                if calculate_displacement < displacement {
                    min_draft = mean_draft;
                    mean_draft = (max_draft + min_draft) / 2.0;
                } else {
                    max_draft = mean_draft;
                    mean_draft = (max_draft + min_draft) / 2.0;
                }
            } else {
                return Err("Невозможно выполнить условие удифферентовки судна: ||lcg| - |lcb|| < 0.001 * lbp".to_string());
            }
        }
        Err(
            "Удифферентовка судна не достигнута. Прeвышение максимального количества итераций."
                .to_string(),
        )
    }
}
