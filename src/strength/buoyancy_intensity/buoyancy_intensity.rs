use tracing::instrument;

use super::draft::Draft;
use crate::{
    core::{linear_interpolation::LinearInterpolation, water_density::WaterDensity},
    strength::{
        bonjean_scale::bonjean_scale::BonjeanScale,
        ship::{
            ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
            spatium_functions::SpatiumFunctions,
        },
    },
};

///
/// Интенсивность сил поддержания по длине судна, действующих на погруженную часть корпуса судна.
pub struct BuoyancyIntensity {
    b_i: SpatiumFunctions,
}

impl BuoyancyIntensity {
    ///
    /// Основной конструктор.
    pub fn new(b_i: SpatiumFunctions) -> Self {
        BuoyancyIntensity { b_i }
    }

    ///
    /// Вспомогательный конструктор.
    #[instrument(skip_all, target = "BuoyancyIntensity::constructor")]
    pub fn constructor(
        ship_dimensions: ShipDimensions,
        draft: &Draft,
        bonjean_scale: &BonjeanScale,
        water_density: WaterDensity,
    ) -> Result<BuoyancyIntensity, String> {
        let length_spatium = ship_dimensions.length_spatium();
        let half_spatium_len = length_spatium / 2.0;
        let coordinate_aft = ship_dimensions.coordinate_aft();
        let coordinate_nose = ship_dimensions.coordinate_nose();
        let number_spatiums = ship_dimensions.number_spatiums();
        let mut start_coord = coordinate_aft;
        let mut end_coord = start_coord + ship_dimensions.length_spatium();
        let (aft_draft, nose_draft) = draft.draft(ship_dimensions)?;
        let li = LinearInterpolation::new(aft_draft, nose_draft, coordinate_aft, coordinate_nose);
        let mut buoyancy_intensity =
            SpatiumFunctions::filled_zeros(number_spatiums, ship_dimensions.lbp());
        for i in 0..number_spatiums {
            let abscissa = start_coord + half_spatium_len;
            let draft = li.interpolated_value(abscissa)?;
            let frame_area = bonjean_scale.frame_underwater_area(abscissa, draft)?;
            let load_intensity = -1.0 * water_density.water_density() * frame_area;
            let spatium_func =
                SpatiumFunction::new(i, start_coord, end_coord, load_intensity, load_intensity);
            buoyancy_intensity.add(spatium_func);
            start_coord += length_spatium;
            end_coord += length_spatium;
        }
        Ok(BuoyancyIntensity::new(buoyancy_intensity))
    }

    ///
    /// Возвращает интенсивность сил поддержания судна. Размерность: [т/м].
    pub fn buoyancy_intensity(&self) -> &SpatiumFunctions {
        &self.b_i
    }
}
