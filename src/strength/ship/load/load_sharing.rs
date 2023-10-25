use crate::strength::ship::{ship_dimensions::ShipDimensions, deck_cargo::DeckCargo};

use super::ship_load::ShipLoad;

pub struct LoadSharing {
    deck_cargoes: Vec<DeckCargo>,
    ship_dimensions: ShipDimensions
}


// {"value": 68, "center_gravity": {"x": 55, "y": 5.6, "z": 5.6}, "length": 1}

impl LoadSharing {
    pub fn new(deck_cargoes: Vec<DeckCargo>, ship_dimensions: ShipDimensions) -> Self {
        LoadSharing { deck_cargoes, ship_dimensions }
    }

    pub fn shared_loads(&self) -> Vec<ShipLoad> {
        for deck_cargo in &self.deck_cargoes {
            let length = deck_cargo.length;
            let longitudinal_center_gravity = deck_cargo.center_gravity.x;
            let value = deck_cargo.value;
            let x_1 = longitudinal_center_gravity - (length / 2.0);
            let x_4 = longitudinal_center_gravity + (length / 2.0);
            let spatium_start_index = self.ship_dimensions.spatium_index_by_coordinate(x_1);
            let spatium_end_index = self.ship_dimensions.spatium_index_by_coordinate(x_4);
            let x_2 = self.ship_dimensions.spatium_end_coordinate(spatium_start_index);
            let x_3 = self.ship_dimensions.spatium_start_coordinate(spatium_end_index);


        }
        todo!()
    }


}