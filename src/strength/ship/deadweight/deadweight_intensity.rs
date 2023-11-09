use log::warn;
use serde::Deserialize;
use crate::{strength::ship::{load::{shiploads::Shiploads, load_sharing::LoadSharing, shipload::Shipload, load_spread::LoadSpread}, spatium_functions::SpatiumFunctions, ship_dimensions::ShipDimensions}, core::json_file::JsonFile};

#[derive(Deserialize, Debug)]
pub struct DeadweightIntensity {
    load_sharing: LoadSharing,
    ship_dimensions: ShipDimensions
}


impl DeadweightIntensity {
    pub fn new(load_sharing: LoadSharing, ship_dimensions: ShipDimensions) -> Self {
        DeadweightIntensity { load_sharing, ship_dimensions }
    }

    pub fn deadweight_intensity(&self) {
        let number_spatiums = self.ship_dimensions.number_spatiums();
        let length_spatium = self.ship_dimensions.length_spatium();
        let length_between_perpendiculars = self.ship_dimensions.length_between_perpendiculars();
        let spatium_functions = SpatiumFunctions::filled_zeros(number_spatiums, length_spatium, length_between_perpendiculars);
        let shiploads = self.load_sharing.shared_loads();
        for shipload in shiploads.iter() {
            todo!()

        }
    }

    ///
    /// Compute the load intensity.
    fn intensity(&self, shipload: &Shipload) {
        match shipload.spread(&self.ship_dimensions) {
            LoadSpread::WithinOneSpatium => {
                let max_intensity = |c_min: f64| { shipload.value() * (0.5 + (c_min / self.ship_dimensions.length_spatium())) / ship_dimensions.length_spatium() };
                let min_intensity = |c_min: f64| { shipload.value() * (0.5 - (c_min / self.ship_dimensions.length_spatium())) / ship_dimensions.length_spatium() };
                let (distance_left, distance_right) = shipload.distances_to_frames(ship_dimensions);
                let spatium_start_index = shipload.spatium_start_index(ship_dimensions);
                let mut load_component_intensity = vec![];
                if (distance_left > distance_right) && (self.longitudinal_center_gravity() + ship_dimensions.length_spatium() < ship_dimensions.coordinate_bow()) {
                    debug!("Load.intensity | Центр тяжести груза ближе к правому шпангоуту теоретической шпации. c_right={}, c_left={}", distance_right, distance_left);
                    let f_x = max_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_right);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index + 1, ship_dimensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    Ok(load_component_intensity)
                } else if (distance_right > distance_left ) && (self.longitudinal_center_gravity() - ship_dimensions.length_spatium()) > ship_dimensions.coordinate_aft() {
                    debug!("Load.intensity | Центр тяжести груза ближе к левому шпангоуту теоретической шпации. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = max_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    let f_x = min_intensity(distance_left);
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index - 1, ship_dimensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    Ok(load_component_intensity)
                } else {
                    debug!("Load.intensity | Вес груза распределяем на всю теоретическую шпацию. c_right = {}, c_left = {}", distance_right, distance_left);
                    let f_x = self.value / ship_dimensions.length_spatium();
                    let spatium_function = SpatiumFunction::from_id(spatium_start_index, ship_dimensions, f_x, f_x);
                    load_component_intensity.push(spatium_function);
                    debug!("Saptiums are under the load {:#?}", load_component_intensity);
                    Ok(load_component_intensity)
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
                let mut load_intensity: Vec<SpatiumFunction> = vec![];
                let spatium_function = SpatiumFunction::from_id(spatium_id as i64, ship_dimensions, f_x, f_x);
                load_intensity.push(spatium_function);

                let f_x = -((0.5 + (distance / ship_dimensions.length_spatium())) * self.value) / ship_dimensions.length_spatium();
                let spatium_function = SpatiumFunction::from_id(next_spatium_id as i64, ship_dimensions, f_x, f_x);
                load_intensity.push(spatium_function);
                debug!("Saptiums are under the load {:#?}", load_intensity);
                Ok(load_intensity)
            },
            LoadSpread::WithinManySpatiums => {
                Err("The load extends beyond the boundaries of the spatium.
                The load should spreads within one spatium, but now the load spread within many spatiums".to_string())
            }
        }
    }
}

