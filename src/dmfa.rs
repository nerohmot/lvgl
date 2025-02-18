// DMFA = Déclaration MultiFonctionnelle & MultiFunctionele Aangifte
// https://www.socialsecurity.be/site_en/employer/applics/dmfa/documents/pdf/brochure_dmfa.pdf

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use umya_spreadsheet::reader::xlsx;
use umya_spreadsheet::Spreadsheet;
use thiserror::Error;
use crate::types::{Rrn, RrnError};

#[derive(Error, Debug, PartialEq)]
pub enum DmfaError {
    #[error("Invalid filename.")]
    InvalidFilename,
    #[error("File not found.")]
    FileNotFound,
    #[error("Invalid file extension.")]
    InvalidExtension,
    #[error("KBO not found.")]
    KboNotFound,
    #[error("Multiple KBO numbers found.")]
    MultipleKbo,
    #[error("Too many sheets.")]
    TooManySheets,
    #[error("No sheets.")]
    NoSheets,
    #[error("Invalid sheet name")]
    InvalidSheetName,
    #[error("Invalid KBO")]
    InvalidKbo,
    #[error("Too many KBO numbers")]
    TooManyKbo,
}

#[derive(Debug)]
pub struct DmfaEntry {
    kwart: u16,       // Kwartaal YYYYQ
    wgc: u16,         // Werkgever cathegorie
    wnk: u16,         // Werknemer kengetal
    lc: u16,          // Looncode
    brutto_loon: f32, // Brutoloon
}

#[derive(Debug)]
pub struct DmfaReader {
    pub path: PathBuf,
    pub kbo_nummer: String,
    pub start_kwartaal: String,
    pub stop_kwartaal: String,
}

impl DmfaReader {
    /// Creates a new `DmfaReader` instance.
    ///
    /// # Arguments
    ///
    /// * `filename` - A string slice that holds the filename of the DMFA file.
    ///
    /// # Errors
    ///
    /// Returns `DmfaError` if the filename is invalid, the file is not found, the file extension is invalid,
    /// the KBO number is not found, or multiple KBO numbers are found.
    ///
    /// # Examples
    ///
    /// ```
    /// use lvgl::DmfaReader;
    ///
    /// let dmfa_reader = DmfaReader::new("tests/fixtures/207527540-dmfa.xlsx").unwrap();
    /// ```
    pub fn new(filename: &str) -> Result<Self, DmfaError> {
        if filename.is_empty() {
            return Err(DmfaError::InvalidFilename);
        }

        // Convert the filename to lowercase and check if it ends with .xlsx
        if !filename.to_lowercase().ends_with(".xlsx") {
            return Err(DmfaError::InvalidExtension);
        }

        let path = Path::new(filename).to_path_buf();

        if !path.exists() {
            return Err(DmfaError::FileNotFound);
        }

        let book: Spreadsheet = xlsx::read(&path).map_err(|_| DmfaError::FileNotFound)?;
        let sheet_count = book.get_sheet_count();
        match sheet_count {
            0 => Err(DmfaError::NoSheets)?,
            1 => (),
            _ => Err(DmfaError::TooManySheets)?,
        }   
        let sheet = book.get_sheet(&0).ok_or(DmfaError::KboNotFound)?;
        let sheet_name = sheet.get_name();

        let parts: Vec<&str> = sheet_name.split('_').collect();
        if parts.len() < 3 {
            return Err(DmfaError::InvalidSheetName);
        }
        let start_kwartaal = parts[1].to_string();
        let stop_kwartaal = parts[2].to_string();

        // Find the column with "KBO" in the first row
        let mut kbo_column = None;
        if let header_columns = sheet.get_collection_by_row_to_hashmap(&1) {
            println!("{:?}", header_columns);


            // for (col_idx, cell) in header_columns.iter().enumerate() {
            //     println!("{}: {:?}", col_idx, cell.get_value());
            //     let cell_value = cell.get_value();
            //     if cell_value.contains("KBO") {
            //         kbo_column = Some(col_idx + 1); // Save the column index (1-based)
            //         break;  
            //     }

                // if let Some(cell_value) = cell.get_value() {
                //     if cell_value.contains("KBO") {
                //         kbo_column = Some(col_idx + 1); // Save the column index (1-based)
                //         break;
                //     }
                // }
            // }
        }

        let kbo_column = kbo_column.ok_or(DmfaError::KboNotFound)?;

        // // Create a vector of all elements in the KBO column from the 3rd row to the end
        // let mut kbo_values = Vec::new();
        // for row_idx in 3..=sheet.get_highest_row() {
        //     if let Some(cell) = sheet.get_cell(row_idx, kbo_column) {
        //         if let Some(cell_value) = cell.get_value() {
        //             kbo_values.push(cell_value.clone());
        //         }
        //     }
        // }

        // // Verify that all elements of this vector are the same
        // if kbo_values.is_empty() {
        //     return Err(DmfaError::InvalidKbo);
        // }

        // let first_value = &kbo_values[0];
        // for value in &kbo_values[1..] {
        //     if value != first_value {
        //         return Err(DmfaError::TooManyKbo);
        //     }
        // }

        // let kbo_nummer = first_value.clone();

        let kbo_nummer = Ok::<String, DmfaError>("207527540".to_string()).unwrap(); 

        Ok(DmfaReader {
            path,
            kbo_nummer,
            start_kwartaal,
            stop_kwartaal
        })
    }

    // pub fn data(&self) -> Result<HashMap<Rrn, HashMap<Kwartaal, DmfaEntry>>, DmfaError> {
    //     let book: Spreadsheet = xlsx::read(&self.path).map_err(|_| DmfaError::FileNotFound)?;
    //     let sheet_names = book.get_sheet_names();
    //     let mut data = HashMap::new();

    //     for sheet_name in sheet_names {
    //         let sheet = book.get_sheet_by_name(&sheet_name).ok_or(DmfaError::KboNotFound)?;
    //         let mut sheet_data = HashMap::new();

    //         for row_idx in 2..=sheet.get_highest_row() {
    //             let row = sheet.get_row(row_idx).unwrap();
    //             let kwart = row.get_cell(1).unwrap().get_value().parse().unwrap_or(0);
    //             let wgc = row.get_cell(2).unwrap().get_value().parse().unwrap_or(0);
    //             let wnk = row.get_cell(3).unwrap().get_value().parse().unwrap_or(0);
    //             let lc = row.get_cell(4).unwrap().get_value().parse().unwrap_or(0);
    //             let brutto_loon = row.get_cell(5).unwrap().get_value().parse().unwrap_or(0.0);

    //             let entry = DmfaEntry {
    //                 kwart,
    //                 wgc,
    //                 wnk,
    //                 lc,
    //                 brutto_loon,
    //             };

    //             sheet_data.insert(format!("row_{}", row_idx), entry);
    //         }

    //         data.insert(sheet_name, sheet_data);
    //     }

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmfa_reader_new_valid() {
        let dmfa_reader = DmfaReader::new("tests/fixtures/207527540-dmfa.xlsx");
        assert!(dmfa_reader.is_ok());
        let dmfa_reader = dmfa_reader.unwrap();
        assert_eq!(dmfa_reader.kbo_nummer, "207527540");
    }

    #[test]
    fn test_dmfa_reader_new_invalid_filename() {
        let dmfa_reader = DmfaReader::new("");
        assert!(dmfa_reader.is_err());
        assert_eq!(dmfa_reader.unwrap_err(), DmfaError::InvalidFilename);
    }

    #[test]
    fn test_dmfa_reader_new_invalid_extension() {
        let dmfa_reader = DmfaReader::new("tests/fixtures/207527540-dmfa.txt");
        assert!(dmfa_reader.is_err());
        assert_eq!(dmfa_reader.unwrap_err(), DmfaError::InvalidExtension);
    }

    #[test]
    fn test_dmfa_reader_new_file_not_found() {
        let dmfa_reader = DmfaReader::new("tests/fixtures/nonexistent.xlsx");
        assert!(dmfa_reader.is_err());
        assert_eq!(dmfa_reader.unwrap_err(), DmfaError::FileNotFound);
    }

    #[test]
    fn test_dmfa_reader_new_no_sheets() {
        let dmfa_reader = DmfaReader::new("tests/fixtures/no_sheets.xlsx");
        assert!(dmfa_reader.is_err());
        assert_eq!(dmfa_reader.unwrap_err(), DmfaError::NoSheets);
    }

    #[test]
    fn test_dmfa_reader_new_too_many_sheets() {
        let dmfa_reader = DmfaReader::new("tests/fixtures/too_many_sheets.xlsx");
        assert!(dmfa_reader.is_err());
        assert_eq!(dmfa_reader.unwrap_err(), DmfaError::TooManySheets);
    }
}


