use log::debug;

use crate::strength::ship::{ship_dimensions::ShipDimensions, load::shipload::Shipload};



pub struct LoadSharing<'a> {
    ship_dimensions: &'a ShipDimensions,
    shipload: &'a Shipload
}


impl<'a> LoadSharing<'a> {
    pub fn new(ship_dimensions: &'a ShipDimensions, shipload: &'a Shipload) -> Self {
        LoadSharing { ship_dimensions, shipload }
    }

    ///
    /// Share the shipload by spatiums.
    pub fn shared_loads(&self) -> Vec<Shipload> {
        let mut shared_shiploads = vec![];
        let x_1 = self.shipload.load_start_coordinate();
        let x_4 = self.shipload.load_end_coordinate();
        let spatium_start_index = self.ship_dimensions.spatium_index_by_coordinate(x_1);
        let spatium_end_index = self.ship_dimensions.spatium_index_by_coordinate(x_4);
        let x_2 = self.ship_dimensions.spatium_end_coordinate(spatium_start_index);
        let x_3 = self.ship_dimensions.spatium_start_coordinate(spatium_end_index);
        debug!("x_1 = {}, x_2 = {}, x_3 = {}, x_4 = {}", x_1, x_2, x_3, x_4);
        if (x_1.abs() - x_2.abs()).abs() > 0.0 {
            let load = self.shipload.shared_shipload(x_1, x_2);
            shared_shiploads.push(load);
        }
        if (x_4.abs() - x_3.abs()).abs() > 0.0 {
            let load = self.shipload.shared_shipload(x_3, x_4);
            shared_shiploads.push(load);
        }
        let mut load_start_coordinate = x_2;
        let mut load_end_coordinate = x_2 + self.ship_dimensions.length_spatium();
        while load_end_coordinate <= x_3 {
            let load = self.shipload.shared_shipload(load_start_coordinate, load_end_coordinate);
            shared_shiploads.push(load);
            load_start_coordinate += self.ship_dimensions.length_spatium();
            load_end_coordinate += self.ship_dimensions.length_spatium();
        }
        shared_shiploads
    }

}



