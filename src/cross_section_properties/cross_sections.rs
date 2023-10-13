use std::collections::HashMap;
use csv::Reader;
use log::{debug, warn};
use std::fs::File;
use crate::core::csv::CSV;

use super::cross_section::CrossSection;


pub struct CrossSections {
    cross_sections: HashMap<i32, CrossSection>,
}

impl CrossSections {
    pub fn new(cross_sections: HashMap<i32, CrossSection>) -> Self {
        CrossSections { cross_sections }
    }
    pub fn from_csv_parser(mut parser: Reader<File>) -> Option<Result<Self, String>> {
        let mut cross_sections = HashMap::new();
        for result in parser.deserialize::<CrossSection>() {
            match result {
                Ok(cross_section) => { cross_sections.insert(cross_section.id, cross_section); },
                Err(err) => {
                    warn!("CrossSections::from_csv_parser | error: {:?}",err);
                    return Some(Err(err.to_string()));
                }
            }
        }
        if cross_sections.is_empty() {
            debug!("CrossSections::from_csv_parser | Cross sections have not been defined.\n CrossSections:\n {:#?}", cross_sections);
            return None;
        }
        debug!("CrossSections::from_csv_parser | Cross sections have been created successfully.\n CrossSections:\n {:#?}", cross_sections);
        Some(Ok(CrossSections::new(cross_sections)))
    }

    pub fn from_csv_file(file_path: String) -> Option<Result<Self, String>> {
        let input = CSV::new(file_path);
        match input.parser() {
            Ok(parser) => {
                CrossSections::from_csv_parser(parser)
            },
            Err(err) => {
                warn!("CrossSections::from_csv_file | error: {:?}.",err);
                Some(Err(err.to_string()))
            }
        }
    }

    fn cross_section(&self, id:i32) -> Option<&CrossSection> {
        self.cross_sections.get(&id)
    }
}
