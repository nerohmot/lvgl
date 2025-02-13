use std::{fs::File, io::Read, path::Path};
use infer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("File '{0}' does not exist or is not a file.")]
    FileNotFound(String),
    #[error("Unable to open file '{0}'.")]
    FileOpenError(String),
    #[error("Unable to read file '{0}'.")]
    FileReadError(String),
    #[error("File '{0}' is not an XLSX file.")]
    InvalidFileType(String),
}

pub fn is_valid(path: &Path) -> Result<bool, ValidationError> {
    if !path.exists() || !path.is_file() {
        return Err(ValidationError::FileNotFound(path.to_string_lossy().to_string()));
    }

    let mut file = File::open(path).map_err(|_| ValidationError::FileOpenError(path.to_string_lossy().to_string()))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|_| ValidationError::FileReadError(path.to_string_lossy().to_string()))?;

    let kind = infer::get(&buffer).ok_or_else(|| ValidationError::InvalidFileType(path.to_string_lossy().to_string()))?;
    if kind.mime_type() != "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" {
        return Err(ValidationError::InvalidFileType(path.to_string_lossy().to_string()));
    }

    Ok(true)
}

pub fn compare_dmfa_bosa(dmfa: &Path, bosa: &Path) -> Result<(), ValidationError> {
    let dmfa_valid = is_valid(dmfa)?;
    let bosa_valid = is_valid(bosa)?;

    if !dmfa_valid {
        eprintln!("DMFA document must be in XLSX format.");
        return Err(ValidationError::InvalidFileType(dmfa.to_string_lossy().to_string()));
    }

    if !bosa_valid {
        eprintln!("BOSA document must be in XLSX format.");
        return Err(ValidationError::InvalidFileType(bosa.to_string_lossy().to_string()));
    }

    Ok(())
}

pub fn compare_dmfa_cipal(dmfa: &Path, cipal: &Path) -> Result<(), ValidationError> {
    let dmfa_valid = is_valid(dmfa)?;
    let cipal_valid = is_valid(cipal)?;

    if !dmfa_valid {
        eprintln!("DMFA document must be in XLSX format.");
        return Err(ValidationError::InvalidFileType(dmfa.to_string_lossy().to_string()));
    }

    if !cipal_valid {
        eprintln!("CIPAL document must be in XLSX format.");
        return Err(ValidationError::InvalidFileType(cipal.to_string_lossy().to_string()));
    }

    Ok(())
}