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

use super::ship_trimming::ShipTrimming;

///
/// Интенсивность сил поддержания по длине судна, действующие на погруженную часть корпуса судна.
pub(crate) struct BuoyancyIntensity<'a> {
    ship_trimming: ShipTrimming<'a>,
    bonjean_scale: &'a BonjeanScale,
    water_density: WaterDensity,
}

impl<'a> BuoyancyIntensity<'a> {
    ///
    /// Основной конструктор.
    pub fn new(
        ship_trimming: ShipTrimming<'a>,
        bonjean_scale: &'a BonjeanScale,
        water_density: WaterDensity,
    ) -> Self {
        BuoyancyIntensity {
            ship_trimming,
            bonjean_scale,
            water_density,
        }
    }

    ///
    /// Возвращает интенсивность сил поддержания судна [т/м].
    pub fn buoyancy_intensity(
        &self,
        ship_dimensions: &ShipDimensions,
    ) -> Result<SpatiumFunctions, String> {
        let length_spatium = ship_dimensions.length_spatium();
        let half_spatium_len = length_spatium / 2.0;
        let coordinate_aft = ship_dimensions.coordinate_aft();
        let coordinate_nose = ship_dimensions.coordinate_nose();
        let number_spatiums = ship_dimensions.number_spatiums();
        let mut start_coord = coordinate_aft;
        let mut end_coord = start_coord + ship_dimensions.length_spatium();
        let (aft_draft, nose_draft) = self.ship_trimming.trim(ship_dimensions)?;
        let li = LinearInterpolation::new(aft_draft, nose_draft, coordinate_aft, coordinate_nose);
        let mut buoyancy_intensity =
            SpatiumFunctions::filled_zeros(number_spatiums, ship_dimensions.lbp());
        for i in 0..number_spatiums {
            let abscissa = start_coord + half_spatium_len;
            let draft = li.interpolated_value(abscissa)?;
            let frame_area = self.bonjean_scale.frame_underwater_area(abscissa, draft)?;
            let load_intensity = -1.0 * self.water_density.water_density() * frame_area;
            let spatium_func =
                SpatiumFunction::new(i, start_coord, end_coord, load_intensity, load_intensity);
            buoyancy_intensity.add(spatium_func);
            start_coord += length_spatium;
            end_coord += length_spatium;
        }
        Ok(buoyancy_intensity)
    }
}
