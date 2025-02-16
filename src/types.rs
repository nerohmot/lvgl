// RijksRegisterNummer (RRN) is a Belgian national identification number.
use std::fmt;
use thiserror::Error;
use std::num::ParseIntError;

#[derive(PartialEq, Eq)]
pub enum Gender {
    M,
    F,
}

impl fmt::Debug for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::M => write!(f, "Male"),
            Gender::F => write!(f, "Female"),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RrnError {
    #[error("Invalid Rijksregister Nummer Length.")]
    InvalidLength,
    #[error("Invalid Rijksregister Nummer.")]
    InvalidControl,
    #[error("ParseInt error: {0}")]
    ParseIntError(#[from] ParseIntError),
}   

#[derive(Debug, PartialEq, Eq)]
pub struct Rrn {
    rrn: String,
}

impl Rrn {
    /// Creates a new `Rrn` instance.
    ///
    /// # Arguments
    ///
    /// * `rrn` - A string slice that holds the RRN.
    ///
    /// # Errors
    ///
    /// Returns `RrnError::InvalidLength` if the length of the RRN is not 9, 10, or 11 characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use lvgl::Rrn;
    ///
    /// let rrn = Rrn::new("69.10.01-363.59").unwrap();
    /// ```
    pub fn new(rrn: &str) -> Result<Self, RrnError> {
        let mut rrn = rrn.trim().replace(&['.', '-'][..], "");

        match rrn.len() {
            11 => {},
            10 => { rrn = format!("0{}", rrn)},
            9 => { rrn = format!("00{}", rrn)},
            _ => return Err(RrnError::InvalidLength),
        }

        Ok(Rrn { rrn })
    }

    /// Checks the validity of the RRN and determines the gender.
    ///
    /// # Errors
    ///
    /// Returns `RrnError::InvalidControl` if the control number is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use lvgl::{Rrn, Gender};
    ///
    /// let rrn = Rrn::new("69.10.01-363.59").unwrap();
    /// let gender = rrn.check().unwrap();
    /// assert_eq!(gender, Gender::M);
    /// ```
    pub fn check(&self) -> Result<Gender, RrnError> {
        let base = self.rrn.chars().take(9).collect::<String>().parse::<u32>()?;
        let control = self.rrn.chars().skip(9).collect::<String>().parse::<u32>()?;
        let check = 97 - (base % 97);

        if check == control { // Check for pre 2000
            println!("Pre 2000");
            let id = self.rrn.chars().skip(6).take(3).collect::<String>().parse::<u32>()?;

            if id % 2 == 0 {
                return Ok(Gender::F);
            } else {
                return Ok(Gender::M);
            }
        } else { // Check for post 2000
            println!("Post 2000");
            let check2 = 97 - ((base + 2000000000) % 97);

            if check2 == control {
                let id = self.rrn.chars().skip(6).take(3).collect::<String>().parse::<u32>()?;

                if id % 2 == 0 {
                    return Ok(Gender::F);
                } else {
                    return Ok(Gender::M);
                }
            } else {
                return Err(RrnError::InvalidControl);
            }
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum KwartaalError {
    #[error("Invalid Year.")]
    InvalidYear,
    #[error("Invalid Quarter.")]
    InvalidQuarter,
    #[error("Invalid Length.")]
    InvalidLength,
    #[error("ParseInt error: {0}")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(PartialEq, Eq)]
pub struct Kwartaal {
    pub year: u16,
    pub quarter: u8,
}
    
impl Kwartaal {
    pub fn new(kwart: String) -> Result<Self, KwartaalError> {
        let kwart = kwart.trim().replace(&['.', '-'][..], "");
        
        match kwart.len() {
            5 => {},
            _ => return Err(KwartaalError::InvalidLength),
        }

        let year = kwart.chars().take(4).collect::<String>().parse::<u16>()?;
        
        if year < 1970 { // let's use the epoch as a reference
            return Err(KwartaalError::InvalidYear);
        }
        if year > 2100 {
            return Err(KwartaalError::InvalidYear);
        }   

        let quarter = kwart.chars().skip(4).collect::<String>().parse::<u8>()?;

        if quarter < 1 {
            return Err(KwartaalError::InvalidQuarter);
        }

        if quarter > 4 {
            return Err(KwartaalError::InvalidQuarter);
        }

        Ok(Self { year, quarter })
    }
}

impl fmt::Debug for Kwartaal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.year, self.quarter)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum MonthError {
    #[error("Invalid Year.")]
    InvalidYear,
    #[error("Invalid Month.")]
    InvalidMonth,
    #[error("Invalid Length.")]
    InvalidLength,
    #[error("ParseInt error: {0}")]
    ParseIntError(#[from] ParseIntError),

}

#[derive(Debug, PartialEq, Eq)]
pub struct BosaMonth {
    pub year: u16,
    pub month: u8,
}

impl BosaMonth {
    pub fn new(month: String) -> Result<Self, MonthError> {
        let month = month.trim();

        match month.len() {
            6 => {},
            _ => return Err(MonthError::InvalidLength),
        }

        let year = month.chars().take(4).collect::<String>().parse::<u16>()?;

        if year < 1970 { // let's use the epoch as a reference
            return Err(MonthError::InvalidYear);
        }
        if year > 2100 {
            return Err(MonthError::InvalidYear);
        }

        let month = month.chars().skip(4).collect::<String>().parse::<u8>()?;

        if month < 1 {
            return Err(MonthError::InvalidMonth);
        }

        if month > 12 {
            return Err(MonthError::InvalidMonth);
        }

        Ok(Self { year, month })
    }

    pub fn to_kwartaal(&self) -> Kwartaal {
        let quarter = match self.month {
            1 | 2 | 3 => 1,
            4 | 5 | 6 => 2,
            7 | 8 | 9 => 3,
            10 | 11 | 12 => 4,
            _ => 0, // This can never happen
        };

        Kwartaal { year: self.year, quarter }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CipalMonth {
    pub year: u16,
    pub month: u8,
}

impl CipalMonth {
    pub fn new(month: String) -> Result<Self, MonthError> {
        let mut month = month.trim().replace(&['/'][..], "");

        match month.len() {
            5 => {
                month = format!("0{}", month);
            },
            6 => {},
            _ => return Err(MonthError::InvalidLength),
        }

        let year = month.chars().skip(2).take(4).collect::<String>().parse::<u16>()?;

        if year < 1970 { // let's use the epoch as a reference
            return Err(MonthError::InvalidYear);
        }
        if year > 2100 {
            return Err(MonthError::InvalidYear);
        }

        let month = month.chars().take(2).collect::<String>().parse::<u8>()?;

        if month < 1 {
            return Err(MonthError::InvalidMonth);
        }

        if month > 12 {
            return Err(MonthError::InvalidMonth);
        }

        Ok(Self { year, month })
    }

    pub fn to_kwartaal(&self) -> Kwartaal {
        let quarter = match self.month {
            1 | 2 | 3 => 1,
            4 | 5 | 6 => 2,
            7 | 8 | 9 => 3,
            10 | 11 | 12 => 4,
            _ => 0, // This can never happen
        };

        Kwartaal { year: self.year, quarter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rrn_tests {
        use super::*;

        #[test]
        fn test_rrn_valid_male_pre2000() {
            let rrn = Rrn::new("69.10.01-363.59").unwrap();
            let result = rrn.check();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Gender::M);
        }

        #[test]
        fn test_rrn_valid_female_pre2000() {
            let rrn = Rrn::new("95022899874").unwrap();
            let result = rrn.check();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Gender::F);
        }

        #[test]
        fn test_rrn_valid_male_post2000() {
            let rrn = Rrn::new("02022404596").unwrap();
            let result = rrn.check();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Gender::M);
        }

        #[test]
        fn test_rrn_valid_female_post2000() {
            let rrn = Rrn::new("05050413214").unwrap();
            let result = rrn.check();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Gender::F);
        }

        #[test]
        fn test_rrn_valid_very_old_woman() {
            let rrn = Rrn::new("15030630050").unwrap();
            let result = rrn.check();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Gender::F);
        }

        #[test]
        fn test_rrn_invalid_length_too_long() {
            let rrn = Rrn::new("123456789011");
            assert!(rrn.is_err());
            assert_eq!(rrn.unwrap_err(), RrnError::InvalidLength);
        }

        #[test]
        fn test_rrn_invalid_length_too_short() {
            let rrn = Rrn::new("1234678");
            assert!(rrn.is_err());
            assert_eq!(rrn.unwrap_err(), RrnError::InvalidLength);
        }

        #[test]
        fn test_rrn_invalid_control() {
            let rrn = Rrn::new("95022899873").unwrap();
            let result = rrn.check();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), RrnError::InvalidControl);
        }

        #[test]
        fn test_rrn_equality() {
            let rrn1 = Rrn::new("69.10.01-363.59").unwrap();
            let rrn2 = Rrn::new("69100136359").unwrap();
            assert_eq!(rrn1, rrn2);
        }

        #[test]
        fn test_rrn_inequality() {
            let rrn1 = Rrn::new("69.10.01-363.59").unwrap();
            let rrn2 = Rrn::new("95022899874").unwrap();
            assert_ne!(rrn1, rrn2);
        }
    }

    mod gender_tests {
        use super::*;

        #[test]
        fn test_debug_gender_male() {
            let gender = Gender::M;
            assert_eq!(format!("{:?}", gender), "Male");
        }

        #[test]
        fn test_debug_gender_female() {
            let gender = Gender::F;
            assert_eq!(format!("{:?}", gender), "Female");
        }
    }

    mod kwartaal_tests {
        use super::*;

        #[test]
        fn test_kwartaal_new_valid() {
            let kwartaal = Kwartaal::new("20211".to_string()).unwrap();
            assert_eq!(kwartaal.year, 2021);
            assert_eq!(kwartaal.quarter, 1);
        }

        #[test]
        fn test_kwartaal_new_invalid_length() {
            let result = Kwartaal::new("2021".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), KwartaalError::InvalidLength);
        }

        #[test]
        fn test_kwartaal_new_invalid_year_non_numerical() {
            let result = Kwartaal::new("abcd1".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "ParseInt error: invalid digit found in string");
        }

        #[test]
        fn test_kwartaal_new_invalid_quarter_non_numerical() {
            let result = Kwartaal::new("2021x".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "ParseInt error: invalid digit found in string");
        }

        #[test]
        fn test_kwartaal_new_invalid_year_pre_epoch() {
            let result = Kwartaal::new("19691".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), KwartaalError::InvalidYear);
        }

        #[test]
        fn test_kwartaal_new_invalid_quarter_too_low() {
            let result = Kwartaal::new("20210".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), KwartaalError::InvalidQuarter);
        }

        #[test]
        fn test_kwartaal_new_invalid_quarter_too_high() {
            let result = Kwartaal::new("20215".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), KwartaalError::InvalidQuarter);
        }

        #[test]
        fn test_debug_kwartaal() {
            let kwartaal = Kwartaal::new("20211".to_string()).unwrap();
            assert_eq!(format!("{:?}", kwartaal), "20211");
        }
    }

    mod bosa_month_tests {
        use super::*;

        #[test]
        fn test_bosa_month_new_valid() {
            let month = BosaMonth::new("202101".to_string()).unwrap();
            assert_eq!(month.year, 2021);
            assert_eq!(month.month, 1);
        }

        #[test]
        fn test_bosa_month_new_invalid_length() {
            let result = BosaMonth::new("2021".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidLength);
        }

        #[test]
        fn test_bosa_month_new_invalid_year_non_numerical() {
            let result = BosaMonth::new("abcd01".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "ParseInt error: invalid digit found in string");
        }

        #[test]
        fn test_bosa_month_new_invalid_month_non_numerical() {
            let result = BosaMonth::new("2021xx".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "ParseInt error: invalid digit found in string");
        }

        #[test]
        fn test_bosa_month_new_invalid_year_pre_epoch() {
            let result = BosaMonth::new("196901".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidYear);
        }

        #[test]
        fn test_bosa_month_new_invalid_month_too_low() {
            let result = BosaMonth::new("202100".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidMonth);
        }

        #[test]
        fn test_bosa_month_new_invalid_month_too_high() {
            let result = BosaMonth::new("202113".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidMonth);
        }

        #[test]
        fn test_bosa_month_to_kwartaal() {
            let month = BosaMonth::new("202101".to_string()).unwrap();
            let kwartaal = month.to_kwartaal();
            assert_eq!(kwartaal.year, 2021);
            assert_eq!(kwartaal.quarter, 1);
        }
    }

    mod cipal_month_tests {
        use super::*;

        #[test]
        fn test_cipal_month_new_valid() {
            let month = CipalMonth::new("01/2021".to_string()).unwrap();
            assert_eq!(month.year, 2021);
            assert_eq!(month.month, 1);
        }

        #[test]
        fn test_cipal_month_new_invalid_length() {
            let result = CipalMonth::new("01/21".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidLength);
        }

        #[test]
        fn test_cipal_month_new_invalid_year_non_numerical() {
            let result = CipalMonth::new("01/abcd".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "ParseInt error: invalid digit found in string");
        }

        #[test]
        fn test_cipal_month_new_invalid_month_non_numerical() {
            let result = CipalMonth::new("xx/2021".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().to_string(), "ParseInt error: invalid digit found in string");
        }

        #[test]
        fn test_cipal_month_new_invalid_year_pre_epoch() {
            let result = CipalMonth::new("01/1969".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidYear);
        }

        #[test]
        fn test_cipal_month_new_invalid_month_too_low() {
            let result = CipalMonth::new("00/2021".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidMonth);
        }

        #[test]
        fn test_cipal_month_new_invalid_month_too_high() {
            let result = CipalMonth::new("13/2021".to_string());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), MonthError::InvalidMonth);
        }

        #[test]
        fn test_cipal_month_to_kwartaal() {
            let month = CipalMonth::new("01/2021".to_string()).unwrap();
            let kwartaal = month.to_kwartaal();
            assert_eq!(kwartaal.year, 2021);
            assert_eq!(kwartaal.quarter, 1);
        }
    }
}