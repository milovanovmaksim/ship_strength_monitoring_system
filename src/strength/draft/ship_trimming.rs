use crate::strength::{
    bonjean_scale::lcb::LCB,
    buoyancy_intensity::lcg::LCG,
    hydrostatic_curves::{
        hydrostatic_curves::HydrostaticCurves, hydrostatic_typedata::HydrostaticTypeData,
    },
    ship::ship_dimensions::ShipDimensions,
};

///
/// Удифферентовка судна на тихой воде.
/// Parameters:
///    lcb - абсцисса центра велечины,
///    lcg - центр тяжести судна,
///    hydrostatic_curves - гидростатические кривые,
pub(crate) struct ShipTrimming<'a> {
    lcb: &'a LCB<'a>,
    lcg: &'a LCG<'a>,
    hydrostatic_curves: &'a HydrostaticCurves,
}

impl<'a> ShipTrimming<'a> {
    ///
    /// Основной конструктор.
    pub fn new(
        lcb: &'a LCB<'a>,
        lcg: &'a LCG<'a>,
        hydrostatic_curves: &'a HydrostaticCurves,
    ) -> Self {
        ShipTrimming {
            lcb,
            lcg,
            hydrostatic_curves,
        }
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

    ///
    /// Удифферентовка судна методом последовательных приближений.
    /// Возвращает осадки кормы и носа судна если условие удифферентовки
    /// судна (||lcg| - |lcb|| < 0.001 * lbp) достижимо, иначе None.
    pub fn trimming(
        &self,
        mean_draft: f64,
        ship_dimensions: &ShipDimensions,
    ) -> Result<Option<(f64, f64)>, String> {
        let lcg = self.lcg.lcg()?;
        let lcf = self
            .hydrostatic_curves
            .get_data_by_draft(mean_draft, HydrostaticTypeData::LCF)?
            .unwrap(); // TODO: проверить на None
        let lbp = ship_dimensions.lbp();
        let similarity_coefficient = self.similarity_coefficient(lcf, lbp);
        let mut max_draft = self.hydrostatic_curves.max_draft();
        let mut min_draft = self.hydrostatic_curves.min_draft();
        let mut nose_draft = mean_draft;
        let mut aft_draft = mean_draft;
        let mut lcb = self.lcb.lcb(aft_draft, nose_draft)?;
        for _ in 0..50 {
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
            if (lcg.abs() - lcb.abs()).abs() < 0.001 * lbp {
                return Ok(Some((aft_draft, nose_draft)));
            }
        }
        return Ok(None);
    }
}
