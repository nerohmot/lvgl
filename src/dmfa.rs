// DMFA = Déclaration MultiFonctionnelle & MultiFunctionele Aangifte
// https://www.socialsecurity.be/site_en/employer/applics/dmfa/documents/pdf/brochure_dmfa.pdf

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use umya_spreadsheet::reader::xlsx;
use umya_spreadsheet::Spreadsheet;
use thiserror::Error;
use crate::rrn::{Rrn, RrnError};

#[derive(Error, Debug)]
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

        // Load the Excel file
        let book: Spreadsheet = xlsx::read(&path).map_err(|_| DmfaError::FileNotFound)?;

        // Retrieve the names of all sheets
        let sheet_names = book.get_sheet_names();
        if sheet_names.is_empty() {
            return Err(DmfaError::KboNotFound);
        }

        // Access the first sheet by its name
        let sheet = book.get_sheet_by_name(&sheet_names[0]).ok_or(DmfaError::KboNotFound)?;

        // Iterate through the first row (header) to find "KBO"
        let mut kbo_column = None;
        for (col_idx, cell) in sheet.get_row(1).unwrap().get_cells().iter().enumerate() {
            if let Some(cell_value) = cell.get_value() {
                if cell_value.contains("KBO") {
                    kbo_column = Some(col_idx + 1); // Save the column index (1-based)
                    break;
                }
            }
        }

        let kbo_column = kbo_column.ok_or(DmfaError::KboNotFound)?;

        // Check if the cells from line 3 to the end all contain the same value
        let mut kbo_nummer = None;
        for row_idx in 3..=sheet.get_highest_row() {
            let cell = sheet.get_cell(row_idx, kbo_column).unwrap();
            if let Some(cell_value) = cell.get_value() {
                match kbo_nummer {
                    Some(ref value) if value != cell_value => return Err(DmfaError::MultipleKbo),
                    None => kbo_nummer = Some(cell_value.clone()),
                    _ => (),
                }
            }
        }

        let kbo_nummer = kbo_nummer.ok_or(DmfaError::KboNotFound)?;

        Ok(DmfaReader {
            path,
            kbo_nummer,
        })
    }

    pub fn data(&self) -> Result<HashMap<Rrn, HashMap<String, DmfaEntry>>, DmfaError> {
        let book: Spreadsheet = xlsx::read(&self.path).map_err(|_| DmfaError::FileNotFound)?;
        let sheet_names = book.get_sheet_names();
        let mut data = HashMap::new();

        for sheet_name in sheet_names {
            let sheet = book.get_sheet_by_name(&sheet_name).ok_or(DmfaError::KboNotFound)?;
            let mut sheet_data = HashMap::new();

            for row_idx in 2..=sheet.get_highest_row() {
                let row = sheet.get_row(row_idx).unwrap();
                let kwart = row.get_cell(1).unwrap().get_value().parse().unwrap_or(0);
                let wgc = row.get_cell(2).unwrap().get_value().parse().unwrap_or(0);
                let wnk = row.get_cell(3).unwrap().get_value().parse().unwrap_or(0);
                let lc = row.get_cell(4).unwrap().get_value().parse().unwrap_or(0);
                let brutto_loon = row.get_cell(5).unwrap().get_value().parse().unwrap_or(0.0);

                let entry = DmfaEntry {
                    kwart,
                    wgc,
                    wnk,
                    lc,
                    brutto_loon,
                };

                sheet_data.insert(format!("row_{}", row_idx), entry);
            }

            data.insert(sheet_name, sheet_data);
        }

        Ok(data)
    }
}


