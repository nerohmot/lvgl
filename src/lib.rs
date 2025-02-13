// Module: lib
use std::{fs::File, io::Read, path::Path};
use infer;
use regex::Regex;
use thiserror::Error;
use std::fmt::format;

enum Geslacht {
    M,
    F,
}

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
    #[error("Invalid Rijksregister Nummer.")]
    InvalidRijksregisterNummer,
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

fn kbo_from_dmfa(dmfa: &Path) -> Result<u32, ValidationError> {
    let dmfa_valid = is_valid(dmfa)?;

    if !dmfa_valid {
        eprintln!("DMFA document must be in XLSX format.");
        return Err(ValidationError::InvalidFileType(dmfa.to_string_lossy().to_string()));
    }

    Ok(999_999_999)
}   

fn kbo_from_cipal(cipal: &Path) -> Result<u32, ValidationError> {
    let cipal_valid = is_valid(cipal)?;

    if !cipal_valid {
        eprintln!("CIPAL document must be in XLSX format.");
        return Err(ValidationError::InvalidFileType(cipal.to_string_lossy().to_string()));
    }

    Ok(())
}   

fn kbo_from_bosa(bosa: &Path) -> Result<u32, ValidationError> {
    let bosa_valid = is_valid(bosa)?;

    if !bosa_valid {
        eprintln!("BOSA document must be in XLSX format.");
        return Err(ValidationError::InvalidFileType(bosa.to_string_lossy().to_string()));
    }

    Ok(999_999_999)
}   

fn check_rijksregister_nummer(rijksregister_nummer: &str) -> Result<Geslacht, ValidationError> {
    let re = Regex::new(r"^(\d{2})\.?(\d{2})\.?(\d{2})-?(\d{3})\.?(\d{2})$")?;

    if let Some(caps) = re.captures(rijksregister_nummer) {
        let jj = caps.get(1).map_or(0, |m| m as u32);
        let mm = caps.get(2).map_or(0, |m| m as u32);
        let dd = caps.get(3).map_or(0, |m| m as u32);
        let id = caps.get(4).map_or(0, |m| m as u32);
        let ctrl = caps.get(5).map_or(0, |m| m as u32);
        let base = write!("{:02}{:02}{:02}{:03}",jj, mm, dd, id) as u32;
        let check = 97 - (base % 97);
        if check == ctrl { // check of geboren voor 2000
            if id % 2 == 0 {
                return Ok(Geslacht::F);
            } else {
                return Ok(Geslacht::M);
            }
        } else { // check of geboren in of na 2000
            let base2 = match format(format_args!("2{jj:02}{mm:02}{dd:02}{id:03}")).parse::<u32>() {
                Ok(v) => v,
                Err(_) => return Err(ValidationError::InvalidRijksregisterNummer),
            };
            let check2 = 97 - (base2 % 97);
            if check2 == ctrl {
                if id % 2 == 0 {
                    return Ok(Geslacht::F);
                } else {
                    return Ok(Geslacht::M);
                }
            } else {
                return Err(ValidationError::InvalidRijksregisterNummer);
            }
        }
    } else {
        return Err(ValidationError::InvalidRijksregisterNummer);
    }
    let brol = "2345" as u32;
}