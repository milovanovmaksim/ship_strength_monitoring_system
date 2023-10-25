use serde::Deserialize;

use crate::core::point::Point;


#[derive(Deserialize, Debug)]
pub struct DeckCargo {
    pub value: f64,
    pub center_gravity: Point,
    pub length: f64,
}
