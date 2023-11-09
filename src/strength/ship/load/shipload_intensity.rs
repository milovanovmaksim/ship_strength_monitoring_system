use log::debug;

use crate::strength::ship::{ship_dimensions::ShipDimensions,
    load::{shipload::Shipload, load_sharing::LoadSharing, load_spread::LoadSpread},
    spatium_function::SpatiumFunction};



struct ShiploadIntensity<'a> {
    shipload: &'a Shipload,
    ship_dimensions: &'a ShipDimensions
}

impl<'a> ShiploadIntensity<'a> {
    pub fn new(shipload: &'a Shipload, ship_dimensions: &'a ShipDimensions) -> Self {
        ShiploadIntensity { shipload, ship_dimensions }
    }

    pub fn shipload_intensity(&self) -> Vec<SpatiumFunction> {
        match self.shipload.spread(self.ship_dimensions) {
            LoadSpread::WithinManySpatiums => {
                let mut shipload_intensity = vec![];
                let load_sharing = LoadSharing::new(self.ship_dimensions, self.shipload);
                let shiploads = load_sharing.shared_loads();
                for shipload in shiploads.iter() {
                    let spatium_functions = self._shipload_intensity(shipload);
                    shipload_intensity.extend(spatium_functions);
                }
                shipload_intensity
            },
            _ => {
                self._shipload_intensity(self.shipload)
            }
        }
    }

    ///
    /// Compute the shipload intensity.
    fn _shipload_intensity(&self, shipload: &Shipload) -> Vec<SpatiumFunction> {
        match shipload.spread(self.ship_dimensions) {
            LoadSpread::WithinOneSpatium => {
                let max_intensity = |c_min: f64| { shipload.value() * (0.5 + (c_min / ship_dimensions.length_spatium())) / ship_dimensions.length_spatium() };
                let min_intensity = |c_min: f64| { self.value * (0.5 - (c_min / ship_dimensions.length_spatium())) / ship_dimensions.length_spatium() };
                let (distance_left, distance_right) = self.distances_to_frames(ship_dimensions);
                let spatium_start_index = self.spatium_start_index(ship_dimensions);
                let mut shipload_intensity = vec![];
                if (distance_left > distance_right) && (self.longitudinal_center_gravity() + ship_dimensions.length_spatium() < ship_dimensions.coordinate_bow()) {
                    debug!("Load.intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
                    let f_x = max_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index + 1, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", shipload_intensity);
                    shipload_intensity
                } else if (distance_right > distance_left ) && (self.longitudinal_center_gravity() - ship_dimensions.length_spatium()) > ship_dimensions.coordinate_aft() {
                    debug!("Load.intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = max_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index - 1, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", shipload_intensity);
                    shipload_intensity
                } else {
                    debug!("Load.intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = self.value / ship_dimensions.length_spatium();
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    shipload_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", shipload_intensity);
                    shipload_intensity
                }
            },
            LoadSpread::OutsideLeftmostFrame | LoadSpread::OutsideRightmostFrame => {
                let (spatium_id, next_spatium_id, distance) = {
                    if self.load_start_coordinate() < ship_dimensions.coordinate_aft() && self.load_end_coordinate() <= ship_dimensions.coordinate_aft() {
                        let distance = (ship_dimensions.coordinate_aft().abs() - self.longitudinal_center_gravity().abs()).abs();
                        (0, 1, distance)
                    } else {
                        let rightmost_spatium_id = ship_dimensions.number_spatiums() - 1;
                        let distance = (ship_dimensions.coordinate_bow().abs() - self.longitudinal_center_gravity().abs()).abs();
                        (rightmost_spatium_id, rightmost_spatium_id - 1, distance)
                    }
                };
                let f_x = ((1.5 + (distance / ship_dimensions.length_spatium())) * self.value) / ship_dimensions.length_spatium();
                let mut shipload_intensity: Vec<SpatiumFunction> = vec![];
                let spatium_function = SpatiumFunction::from_id(spatium_id as i64, ship_dimensions, f_x, f_x);
                shipload_intensity.push(spatium_function);

                let f_x = -((0.5 + (distance / ship_dimensions.length_spatium())) * self.value) / ship_dimensions.length_spatium();
                let spatium_function = SpatiumFunction::from_id(next_spatium_id as i64, ship_dimensions, f_x, f_x);
                shipload_intensity.push(spatium_function);
                debug!("Saptiums are under the load {:#?}", shipload_intensity);
                shipload_intensity
            },
            LoadSpread::WithinManySpatiums => { unreachable!("The shipload was shared.") }
        }
    }
}