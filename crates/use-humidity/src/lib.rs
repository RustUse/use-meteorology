#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Errors returned by humidity constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HumidityValueError {
    /// Relative humidity must be finite.
    NonFiniteRelativeHumidity(f64),
    /// Relative humidity must stay in `0.0..=100.0`.
    RelativeHumidityOutOfRange(f64),
    /// Specific humidity must be finite.
    NonFiniteSpecificHumidity(f64),
    /// Specific humidity cannot be negative.
    NegativeSpecificHumidity(f64),
}

impl fmt::Display for HumidityValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteRelativeHumidity(value) => {
                write!(formatter, "relative humidity must be finite, got {value}")
            },
            Self::RelativeHumidityOutOfRange(value) => {
                write!(
                    formatter,
                    "relative humidity must be in 0.0..=100.0, got {value}"
                )
            },
            Self::NonFiniteSpecificHumidity(value) => {
                write!(formatter, "specific humidity must be finite, got {value}")
            },
            Self::NegativeSpecificHumidity(value) => {
                write!(
                    formatter,
                    "specific humidity cannot be negative, got {value}"
                )
            },
        }
    }
}

impl Error for HumidityValueError {}

/// Stable humidity kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HumidityKind {
    /// Relative humidity.
    Relative,
    /// Specific humidity.
    Specific,
    /// Absolute humidity.
    Absolute,
    /// Mixing ratio.
    MixingRatio,
    /// Unknown humidity kind.
    Unknown,
    /// Caller-defined humidity kind.
    Custom(String),
}

impl fmt::Display for HumidityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relative => formatter.write_str("relative"),
            Self::Specific => formatter.write_str("specific"),
            Self::Absolute => formatter.write_str("absolute"),
            Self::MixingRatio => formatter.write_str("mixing-ratio"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for HumidityKind {
    type Err = HumidityKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(HumidityKindParseError::Empty);
        }

        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "relative" => Ok(Self::Relative),
            "specific" => Ok(Self::Specific),
            "absolute" => Ok(Self::Absolute),
            "mixing-ratio" | "mixingratio" => Ok(Self::MixingRatio),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing humidity kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HumidityKindParseError {
    /// The humidity kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for HumidityKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("humidity kind cannot be empty"),
        }
    }
}

impl Error for HumidityKindParseError {}

/// Relative humidity stored as a percentage.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RelativeHumidity(f64);

impl RelativeHumidity {
    /// Creates relative humidity from a finite percentage in `0.0..=100.0`.
    ///
    /// # Errors
    ///
    /// Returns [`HumidityValueError`] when the value is invalid.
    pub fn new(percent: f64) -> Result<Self, HumidityValueError> {
        if !percent.is_finite() {
            return Err(HumidityValueError::NonFiniteRelativeHumidity(percent));
        }

        if !(0.0..=100.0).contains(&percent) {
            return Err(HumidityValueError::RelativeHumidityOutOfRange(percent));
        }

        Ok(Self(percent))
    }

    /// Returns the stored percentage.
    #[must_use]
    pub fn percent(&self) -> f64 {
        self.0
    }
}

/// Specific humidity stored as kilograms of water vapor per kilogram of air.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SpecificHumidity(f64);

impl SpecificHumidity {
    /// Creates specific humidity from a finite non-negative value.
    ///
    /// # Errors
    ///
    /// Returns [`HumidityValueError`] when the value is invalid.
    pub fn new(kilograms_per_kilogram: f64) -> Result<Self, HumidityValueError> {
        if !kilograms_per_kilogram.is_finite() {
            return Err(HumidityValueError::NonFiniteSpecificHumidity(
                kilograms_per_kilogram,
            ));
        }

        if kilograms_per_kilogram < 0.0 {
            return Err(HumidityValueError::NegativeSpecificHumidity(
                kilograms_per_kilogram,
            ));
        }

        Ok(Self(kilograms_per_kilogram))
    }

    /// Returns the stored specific humidity value.
    #[must_use]
    pub fn kilograms_per_kilogram(&self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{HumidityKind, HumidityKindParseError, HumidityValueError, RelativeHumidity};
    use core::str::FromStr;

    #[test]
    fn valid_relative_humidity() {
        let value = RelativeHumidity::new(55.0).unwrap();

        assert_eq!(value.percent(), 55.0);
    }

    #[test]
    fn negative_relative_humidity_rejected() {
        assert_eq!(
            RelativeHumidity::new(-1.0),
            Err(HumidityValueError::RelativeHumidityOutOfRange(-1.0))
        );
    }

    #[test]
    fn relative_humidity_above_hundred_rejected() {
        assert_eq!(
            RelativeHumidity::new(101.0),
            Err(HumidityValueError::RelativeHumidityOutOfRange(101.0))
        );
    }

    #[test]
    fn humidity_kind_display_and_parse() {
        assert_eq!(HumidityKind::MixingRatio.to_string(), "mixing-ratio");
        assert_eq!(
            HumidityKind::from_str("relative").unwrap(),
            HumidityKind::Relative
        );
        assert_eq!(
            HumidityKind::from_str(" "),
            Err(HumidityKindParseError::Empty)
        );
    }

    #[test]
    fn custom_humidity_kind() {
        assert_eq!(
            HumidityKind::from_str("dew fraction").unwrap(),
            HumidityKind::Custom(String::from("dew fraction"))
        );
    }
}
