use csv::Reader;
use csv::Error as CSVError;
use log::trace;
use log::warn;
use std::fs::File;

pub struct CSV {
    file_path: String,
}


impl CSV {
    pub fn new(file_path: String) -> CSV {
        CSV { file_path }
    }

    pub fn parser(&self) -> Result<Reader<File>, CSVError> {
        match Reader::from_path(&self.file_path) {
            Ok(parser) => {
                trace!("parser: {:?}", parser);
                Ok(parser)
            },
            Err(err) => {
                warn!("CSV.parser() | error: {:?}",err);
                Err(err)
            }
        }
    }
}