use log::info;

use crate::{
    core::round::Round,
    strength::ship::{
        ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction,
        spatium_functions::SpatiumFunctions,
    },
};

use crate::strength::load::{shipload::Shipload, shiploads::Shiploads};

use super::deadweight_intensity::DeadweightIntensity;

///
/// Интенсивность дедвейта по длине судна.
#[derive(Debug)]
pub struct DeadweightIntensityBuilder<'a> {
    shiploads: &'a Shiploads,
    ship_dimensions: ShipDimensions,
}

impl<'a> DeadweightIntensityBuilder<'a> {
    ///
    /// Основной конструктор.
    pub fn new(shiploads: &'a Shiploads, ship_dimensions: ShipDimensions) -> Self {
        DeadweightIntensityBuilder {
            shiploads,
            ship_dimensions,
        }
    }

    ///
    /// Возвращает интенсивность дедвейта по длине судна т/м.
    pub fn build(&self) -> DeadweightIntensity {
        let number_spatiums = self.ship_dimensions.number_spatiums();
        let length_between_perpendiculars = self.ship_dimensions.lbp();
        let mut spatium_functions =
            SpatiumFunctions::filled_zeros(number_spatiums, length_between_perpendiculars);
        let shiploads = self.shiploads.shared_shiploads(&self.ship_dimensions);
        for shipload in shiploads.into_iter() {
            for spatium_function in self.shipload_intensity(shipload).into_iter() {
                spatium_functions.add(spatium_function);
            }
        }
        DeadweightIntensity::new(spatium_functions)
    }

    ///
    /// Максимальная интенсивность на шпацию от нагрузки, расположенной в пределах одной шпации асимметрично.
    fn max_intensity(&self, c_min: f64, shipload: Shipload) -> f64 {
        shipload.value() * (0.5 + (c_min / self.ship_dimensions.length_spatium()))
            / self.ship_dimensions.length_spatium()
    }

    ///
    /// Минимальная интенсивность на шпацию от нагрузки, расположенной в пределах одной шпации асимметрично.
    fn min_intensity(&self, c_min: f64, shipload: Shipload) -> f64 {
        shipload.value() * (0.5 - (c_min / self.ship_dimensions.length_spatium()))
            / self.ship_dimensions.length_spatium()
    }

    ///
    /// Интенсивность от нагрузки, расположенной в пределах одной шпации асимметрично.
    fn intensity_asymmetric_load(
        &self,
        index: u64,
        next_index: u64,
        distance: f64,
        shipload: Shipload,
    ) -> Vec<SpatiumFunction> {
        let f_x_max_intensity = self.max_intensity(distance, shipload).my_round(2);
        let f_x_min_intensity = self.min_intensity(distance, shipload).my_round(2);
        vec![
            SpatiumFunction::from_id(
                index,
                &self.ship_dimensions,
                f_x_max_intensity.my_round(2),
                f_x_max_intensity.my_round(2),
            ),
            SpatiumFunction::from_id(
                next_index,
                &self.ship_dimensions,
                f_x_min_intensity.my_round(2),
                f_x_min_intensity.my_round(2),
            ),
        ]
    }

    fn intensity_load_located_outside_outer_frames(
        &self,
        shipload: Shipload,
    ) -> Vec<SpatiumFunction> {
        let (spatium_id, next_spatium_id, distance) = {
            if shipload.load_start_coordinate() < self.ship_dimensions.coordinate_aft()
                && shipload.load_end_coordinate() <= self.ship_dimensions.coordinate_aft()
            {
                let distance = (self.ship_dimensions.coordinate_aft().abs()
                    - shipload.longitudinal_center_gravity().abs())
                .abs();
                (0, 1, distance)
            } else {
                let rightmost_spatium_id = self.ship_dimensions.number_spatiums() - 1;
                let distance = (self.ship_dimensions.coordinate_nose().abs()
                    - shipload.longitudinal_center_gravity().abs())
                .abs();
                (rightmost_spatium_id, rightmost_spatium_id - 1, distance)
            }
        };
        let f_x = ((1.5 + (distance / self.ship_dimensions.length_spatium())) * shipload.value())
            / self.ship_dimensions.length_spatium();
        let mut spatium_functions: Vec<SpatiumFunction> = vec![];
        let spatium_function = SpatiumFunction::from_id(
            spatium_id,
            &self.ship_dimensions,
            f_x.my_round(2),
            f_x.my_round(2),
        );
        spatium_functions.push(spatium_function);

        let f_x = -((0.5 + (distance / self.ship_dimensions.length_spatium())) * shipload.value())
            / self.ship_dimensions.length_spatium();
        let spatium_function = SpatiumFunction::from_id(
            next_spatium_id,
            &self.ship_dimensions,
            f_x.my_round(2),
            f_x.my_round(2),
        );
        spatium_functions.push(spatium_function);
        info!("Saptiums are under the load {:#?}", spatium_functions);
        spatium_functions
    }

    ///
    /// Интенсивность от нагрузки, находящейся в пределах крайних шпангоутов судна и в пределах одной шпации.
    fn intensity_load_located_inside_outer_frames(
        &self,
        shipload: Shipload,
    ) -> Vec<SpatiumFunction> {
        let spatium_start_index = self
            .ship_dimensions
            .spatium_index_by_coordinate(shipload.longitudinal_center_gravity());
        let (distance_left, distance_right) = shipload.distances_to_frames(&self.ship_dimensions);

        // Груз расположен в пределах одной теоретической шпации несимметрично ближе к правому шпангоуту теоретической шпации.
        if distance_left / distance_right >= 1.05 {
            info!("Shipload.shipload_intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
            let spatium_functions = self.intensity_asymmetric_load(
                spatium_start_index,
                spatium_start_index + 1,
                distance_right,
                shipload,
            );
            info!("Saptiums are under the load {:#?}", spatium_functions);
            spatium_functions

        // Груз расположен в пределах одной теоретической шпации несимметрично ближе к левому шпангоуту теоретической шпации.
        } else if distance_left / distance_right <= 0.95 {
            info!("Load.shipload_intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
            let spatium_functions = self.intensity_asymmetric_load(
                spatium_start_index,
                spatium_start_index - 1,
                distance_left,
                shipload,
            );
            info!("Saptiums are under the load {:#?}", spatium_functions);
            spatium_functions
        // Груз расположен в пределах одной шпации симметрично.
        } else {
            info!("Shipload.shipload_intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
            let f_x = shipload.value() / self.ship_dimensions.length_spatium();
            let spatium_function = SpatiumFunction::new(
                spatium_start_index,
                shipload.load_start_coordinate(),
                shipload.load_end_coordinate(),
                f_x.my_round(2),
                f_x.my_round(2),
            );
            let spatium_functions = vec![spatium_function];
            info!("Saptiums are under the load {:#?}", spatium_functions);
            spatium_functions
        }
    }

    ///
    /// Определяет интенсивность отдельной нагрузки на теоретические шпации [т/м].
    fn shipload_intensity(&self, shipload: Shipload) -> Vec<SpatiumFunction> {
        // Нгарузка не выходит за пределы крайних шпангоутов.
        if shipload.longitudinal_center_gravity() > self.ship_dimensions.coordinate_aft()
            && shipload.longitudinal_center_gravity() < self.ship_dimensions.coordinate_nose()
        {
            self.intensity_load_located_inside_outer_frames(shipload)

        // Нагрузка вышла за пределы крайних шпангоутов судна.
        } else {
            self.intensity_load_located_outside_outer_frames(shipload)
        }
    }
}
