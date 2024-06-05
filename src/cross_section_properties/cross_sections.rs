use log::{debug, warn};
use std::collections::HashMap;
use std::fs::File;

use super::cross_section::CrossSection;

pub struct CrossSections {
    cross_sections: HashMap<i32, CrossSection>,
}

impl CrossSections {
    pub fn new(cross_sections: HashMap<i32, CrossSection>) -> Self {
        CrossSections { cross_sections }
    }

    fn cross_section(&self, id: i32) -> Option<&CrossSection> {
        self.cross_sections.get(&id)
    }
}
