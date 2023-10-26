use log::debug;

use crate::{strength::ship::{ship_dimensions::ShipDimensions, deck_cargo::DeckCargo}, core::point::Point};

use super::ship_load::ShipLoad;

pub struct LoadSharing {
    deck_cargoes: Vec<DeckCargo>,
    ship_dimensions: ShipDimensions
}


impl LoadSharing {
    pub fn new(deck_cargoes: Vec<DeckCargo>, ship_dimensions: ShipDimensions) -> Self {
        LoadSharing { deck_cargoes, ship_dimensions }
    }

    pub fn shared_shiploads(&self) -> Vec<ShipLoad> {
        let mut shared_loads: Vec<ShipLoad> = vec![];
        for deck_cargo in &self.deck_cargoes {
            let longitudinal_center_gravity = deck_cargo.center_gravity.x;
            let x_1 = longitudinal_center_gravity - (deck_cargo.length / 2.0);
            let x_4 = longitudinal_center_gravity + (deck_cargo.length / 2.0);
            let spatium_start_index = self.ship_dimensions.spatium_index_by_coordinate(x_1);
            let spatium_end_index = self.ship_dimensions.spatium_index_by_coordinate(x_4);
            let x_2 = self.ship_dimensions.spatium_end_coordinate(spatium_start_index);
            let x_3 = self.ship_dimensions.spatium_start_coordinate(spatium_end_index);
            if (x_1.abs() - x_2.abs()).abs() > 0.0 {
                let shipload = self.shipload(x_1, x_2, &deck_cargo);
                shared_loads.push(shipload);
            } else if (x_4.abs() - x_3.abs()).abs() > 0.0 {
                let shipload = self.shipload(x_3, x_4, &deck_cargo);
                shared_loads.push(shipload);
            }
            let mut load_start_coordinate = x_2;
            let mut load_end_coordinate = x_2 + self.ship_dimensions.length_spatium();
            let number_whole_spatiums_under_load = ((x_2 - x_3).abs() / self.ship_dimensions.length_spatium()) as i64;
            for _ in 0..number_whole_spatiums_under_load {
                let shipload = self.shipload(load_start_coordinate, load_end_coordinate, &deck_cargo);
                shared_loads.push(shipload);
                load_start_coordinate += self.ship_dimensions.length_spatium();
                load_end_coordinate += self.ship_dimensions.length_spatium();
            }
        }
        debug!("Shared shiploads {:#?}", shared_loads);
        shared_loads
    }

    fn _shared_shiploads(&self, shipload: ShipLoad) -> Vec<ShipLoad> {
        todo!()
    }

    fn shipload(&self, shipload_start_coordinate: f64, shipload_end_coordinate: f64, deck_cargo: &DeckCargo) -> ShipLoad {
        let shipload_length = (shipload_start_coordinate.abs() - shipload_end_coordinate.abs()).abs();
        let longitudinal_center_gravity = shipload_start_coordinate + (shipload_length / 2.0);
        let center_gravity = Point::new(longitudinal_center_gravity, deck_cargo.center_gravity.y, deck_cargo.center_gravity.z);
        let load_value = (shipload_length / deck_cargo.length) * deck_cargo.value;
        ShipLoad::new(load_value, center_gravity, shipload_length)
    }


}