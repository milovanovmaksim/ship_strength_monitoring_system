use log::debug;

use crate::strength::ship::{load::{shipload::Shipload, load_sharing::LoadSharing}, ship_dimensions::ShipDimensions, spatium_function::SpatiumFunction, spatium_functions::SpatiumFunctions};


///
/// Compute the shipload intensity.
pub(crate) struct ShiploadIntensity<'a> {
    shipload: &'a Shipload,
    ship_dimensions: &'a ShipDimensions,
    deadweight: f64
}

impl<'a> ShiploadIntensity<'a> {
    pub fn new(shipload: &'a Shipload, ship_dimensions: &'a ShipDimensions, deadweight: f64) -> Self {
        ShiploadIntensity { shipload, ship_dimensions , deadweight}
    }

    ///
    /// Return SpatiumFunctions containing the shipload intensity.
    pub fn spatium_functions(&self) -> SpatiumFunctions {
        let mut shipload_intensity = vec![];
        let load_sharing = LoadSharing::new(self.ship_dimensions, self.shipload);
        let shiploads = self.shipload.shared_shiploads(&self.ship_dimensions);
        for shipload in shiploads.iter() {
            let spatium_functions = self.shipload_intensity(shipload);
            shipload_intensity.extend(spatium_functions);
        }
        SpatiumFunctions::new(shipload_intensity)
    }

    ///
    /// Compute the shipload intensity.
    fn shipload_intensity(&self, shipload: &Shipload) -> Vec<SpatiumFunction> {
        if shipload.longitudinal_center_gravity() > self.ship_dimensions.coordinate_aft() && shipload.longitudinal_center_gravity() < self.ship_dimensions.coordinate_bow() {
            if shipload.value() < self.deadweight * 0.01 {
                debug!("Shipload.shipload_intensity | Вес груза распределяем на всю теоретическую шпацию. Shipload value = {} < deadweight = {}", self.shipload.value(), self.deadweight);
                let spatium_start_index = self.ship_dimensions.spatium_index_by_coordinate(shipload.longitudinal_center_gravity());
                let f_x = shipload.value() / self.ship_dimensions.length_spatium();
                let spatium_function = SpatiumFunction::from_id(spatium_start_index, self.ship_dimensions, f_x, f_x);
                let spatium_functions = vec![spatium_function];
                debug!("Saptiums are under the load {:#?}", spatium_functions);
                return spatium_functions;
            }
            let max_intensity = |c_min: f64| { shipload.value() * (0.5 + (c_min / self.ship_dimensions.length_spatium())) / self.ship_dimensions.length_spatium() };
            let min_intensity = |c_min: f64| { shipload.value() * (0.5 - (c_min / self.ship_dimensions.length_spatium())) / self.ship_dimensions.length_spatium() };
            let (distance_left, distance_right) = shipload.distances_to_frames(self.ship_dimensions);
            let spatium_start_index = self.ship_dimensions.spatium_index_by_coordinate(shipload.longitudinal_center_gravity());
            let shipload_intensity_closure = |destance: f64, index: u64, next_index: u64| {
                let mut spatium_functions = vec![];
                let f_x_max_intensity = max_intensity(destance);
                let f_x_min_intensity = min_intensity(destance);
                let spatium_function = SpatiumFunction::from_id(index, self.ship_dimensions, f_x_max_intensity, f_x_max_intensity);
                spatium_functions.push(spatium_function);
                let spatium_function = SpatiumFunction::from_id(next_index, self.ship_dimensions, f_x_min_intensity, f_x_min_intensity);
                spatium_functions.push(spatium_function);
                spatium_functions
            };
            if (distance_left - distance_right >= 0.01) && (shipload.longitudinal_center_gravity() + self.ship_dimensions.length_spatium() < self.ship_dimensions.coordinate_bow()) {
                debug!("Shipload.shipload_intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
                let spatium_functions = shipload_intensity_closure(distance_right, spatium_start_index, spatium_start_index + 1);
                debug!("Saptiums are under the load {:#?}", spatium_functions);
                spatium_functions
            } else if (distance_right - distance_left >= 0.01 ) && (shipload.longitudinal_center_gravity() - self.ship_dimensions.length_spatium()) > self.ship_dimensions.coordinate_aft() {
                debug!("Load.shipload_intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
                let spatium_functions = shipload_intensity_closure(distance_left, spatium_start_index, spatium_start_index - 1);
                debug!("Saptiums are under the load {:#?}", spatium_functions);
                spatium_functions
            } else {
                debug!("Shipload.shipload_intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
                let f_x = shipload.value() / self.ship_dimensions.length_spatium();
                let spatium_function = SpatiumFunction::from_id(spatium_start_index, self.ship_dimensions, f_x, f_x);
                let spatium_functions = vec![spatium_function];
                debug!("Saptiums are under the load {:#?}", spatium_functions);
                spatium_functions
            }
        } else {
            let (spatium_id, next_spatium_id, distance) = {
                if shipload.load_start_coordinate() < self.ship_dimensions.coordinate_aft() && shipload.load_end_coordinate() <= self.ship_dimensions.coordinate_aft() {
                    let distance = (self.ship_dimensions.coordinate_aft().abs() - shipload.longitudinal_center_gravity().abs()).abs();
                    (0, 1, distance)
                } else {
                    let rightmost_spatium_id = self.ship_dimensions.number_spatiums() - 1;
                    let distance = (self.ship_dimensions.coordinate_bow().abs() - shipload.longitudinal_center_gravity().abs()).abs();
                    (rightmost_spatium_id, rightmost_spatium_id - 1, distance)
                }
            };
            let f_x = ((1.5 + (distance / self.ship_dimensions.length_spatium())) * shipload.value()) / self.ship_dimensions.length_spatium();
            let mut spatium_functions: Vec<SpatiumFunction> = vec![];
            let spatium_function = SpatiumFunction::from_id(spatium_id, self.ship_dimensions, f_x, f_x);
            spatium_functions.push(spatium_function);

            let f_x = -((0.5 + (distance / self.ship_dimensions.length_spatium())) * shipload.value()) / self.ship_dimensions.length_spatium();
            let spatium_function = SpatiumFunction::from_id(next_spatium_id, self.ship_dimensions, f_x, f_x);
            spatium_functions.push(spatium_function);
            debug!("Saptiums are under the load {:#?}", spatium_functions);
            spatium_functions
        }
    }
}