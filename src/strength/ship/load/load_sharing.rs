use crate::strength::ship::{ship_dimensions::ShipDimensions, load::shipload::Shipload};



pub struct LoadSharing<'a> {
    ship_dimensions: &'a ShipDimensions,
    shipload: &'a Shipload
}


impl<'a> LoadSharing<'a> {
    pub fn new(ship_dimensions: &'a ShipDimensions, shipload: &'a Shipload) -> Self {
        LoadSharing { ship_dimensions, shipload }
    }

    
}


