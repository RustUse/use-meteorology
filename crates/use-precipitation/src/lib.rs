#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Errors returned by precipitation value constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrecipitationValueError {
    /// Precipitation amount must be finite.
    NonFiniteAmount(f64),
    /// Precipitation amount cannot be negative.
    NegativeAmount(f64),
    /// Precipitation rate must be finite.
    NonFiniteRate(f64),
    /// Precipitation rate cannot be negative.
    NegativeRate(f64),
}

impl fmt::Display for PrecipitationValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteAmount(value) => {
                write!(
                    formatter,
                    "precipitation amount must be finite, got {value}"
                )
            },
            Self::NegativeAmount(value) => {
                write!(
                    formatter,
                    "precipitation amount cannot be negative, got {value}"
                )
            },
            Self::NonFiniteRate(value) => {
                write!(formatter, "precipitation rate must be finite, got {value}")
            },
            Self::NegativeRate(value) => {
                write!(
                    formatter,
                    "precipitation rate cannot be negative, got {value}"
                )
            },
        }
    }
}

impl Error for PrecipitationValueError {}

/// Stable precipitation kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrecipitationKind {
    /// Rain.
    Rain,
    /// Drizzle.
    Drizzle,
    /// Snow.
    Snow,
    /// Sleet.
    Sleet,
    /// Hail.
    Hail,
    /// Freezing rain.
    FreezingRain,
    /// Ice pellets.
    IcePellets,
    /// Graupel.
    Graupel,
    /// Mixed precipitation.
    Mixed,
    /// No precipitation.
    None,
    /// Unknown precipitation kind.
    Unknown,
    /// Caller-defined precipitation kind.
    Custom(String),
}

impl fmt::Display for PrecipitationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rain => formatter.write_str("rain"),
            Self::Drizzle => formatter.write_str("drizzle"),
            Self::Snow => formatter.write_str("snow"),
            Self::Sleet => formatter.write_str("sleet"),
            Self::Hail => formatter.write_str("hail"),
            Self::FreezingRain => formatter.write_str("freezing-rain"),
            Self::IcePellets => formatter.write_str("ice-pellets"),
            Self::Graupel => formatter.write_str("graupel"),
            Self::Mixed => formatter.write_str("mixed"),
            Self::None => formatter.write_str("none"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PrecipitationKind {
    type Err = PrecipitationKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PrecipitationKindParseError::Empty);
        }

        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "rain" => Ok(Self::Rain),
            "drizzle" => Ok(Self::Drizzle),
            "snow" => Ok(Self::Snow),
            "sleet" => Ok(Self::Sleet),
            "hail" => Ok(Self::Hail),
            "freezing-rain" | "freezingrain" => Ok(Self::FreezingRain),
            "ice-pellets" | "icepellets" => Ok(Self::IcePellets),
            "graupel" => Ok(Self::Graupel),
            "mixed" => Ok(Self::Mixed),
            "none" => Ok(Self::None),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing precipitation kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrecipitationKindParseError {
    /// The precipitation kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PrecipitationKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("precipitation kind cannot be empty"),
        }
    }
}

impl Error for PrecipitationKindParseError {}

/// Stable precipitation intensity vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrecipitationIntensity {
    /// Light precipitation.
    Light,
    /// Moderate precipitation.
    Moderate,
    /// Heavy precipitation.
    Heavy,
    /// Extreme precipitation.
    Extreme,
    /// Unknown intensity.
    Unknown,
    /// Caller-defined intensity.
    Custom(String),
}

impl fmt::Display for PrecipitationIntensity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Light => formatter.write_str("light"),
            Self::Moderate => formatter.write_str("moderate"),
            Self::Heavy => formatter.write_str("heavy"),
            Self::Extreme => formatter.write_str("extreme"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PrecipitationIntensity {
    type Err = PrecipitationIntensityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PrecipitationIntensityParseError::Empty);
        }

        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "light" => Ok(Self::Light),
            "moderate" => Ok(Self::Moderate),
            "heavy" => Ok(Self::Heavy),
            "extreme" => Ok(Self::Extreme),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing precipitation intensity fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrecipitationIntensityParseError {
    /// The precipitation intensity was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PrecipitationIntensityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("precipitation intensity cannot be empty"),
        }
    }
}

impl Error for PrecipitationIntensityParseError {}

/// Precipitation amount stored in millimeters.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PrecipitationAmount(f64);

impl PrecipitationAmount {
    /// Creates precipitation amount from a finite non-negative millimeter value.
    ///
    /// # Errors
    ///
    /// Returns [`PrecipitationValueError`] when the amount is invalid.
    pub fn new(millimeters: f64) -> Result<Self, PrecipitationValueError> {
        if !millimeters.is_finite() {
            return Err(PrecipitationValueError::NonFiniteAmount(millimeters));
        }

        if millimeters < 0.0 {
            return Err(PrecipitationValueError::NegativeAmount(millimeters));
        }

        Ok(Self(millimeters))
    }

    /// Returns the stored amount in millimeters.
    #[must_use]
    pub fn millimeters(&self) -> f64 {
        self.0
    }
}

/// Precipitation rate stored in millimeters per hour.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PrecipitationRate(f64);

impl PrecipitationRate {
    /// Creates precipitation rate from a finite non-negative millimeters-per-hour value.
    ///
    /// # Errors
    ///
    /// Returns [`PrecipitationValueError`] when the rate is invalid.
    pub fn new(millimeters_per_hour: f64) -> Result<Self, PrecipitationValueError> {
        if !millimeters_per_hour.is_finite() {
            return Err(PrecipitationValueError::NonFiniteRate(millimeters_per_hour));
        }

        if millimeters_per_hour < 0.0 {
            return Err(PrecipitationValueError::NegativeRate(millimeters_per_hour));
        }

        Ok(Self(millimeters_per_hour))
    }

    /// Returns the stored rate in millimeters per hour.
    #[must_use]
    pub fn millimeters_per_hour(&self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PrecipitationAmount, PrecipitationIntensity, PrecipitationIntensityParseError,
        PrecipitationKind, PrecipitationKindParseError, PrecipitationRate, PrecipitationValueError,
    };
    use core::str::FromStr;

    #[test]
    fn precipitation_kind_display_and_parse() {
        assert_eq!(PrecipitationKind::FreezingRain.to_string(), "freezing-rain");
        assert_eq!(
            PrecipitationKind::from_str("snow").unwrap(),
            PrecipitationKind::Snow
        );
        assert_eq!(
            PrecipitationKind::from_str(" "),
            Err(PrecipitationKindParseError::Empty)
        );
    }

    #[test]
    fn custom_precipitation_kind() {
        assert_eq!(
            PrecipitationKind::from_str("diamond dust").unwrap(),
            PrecipitationKind::Custom(String::from("diamond dust"))
        );
    }

    #[test]
    fn valid_amount() {
        let amount = PrecipitationAmount::new(12.4).unwrap();

        assert_eq!(amount.millimeters(), 12.4);
    }

    #[test]
    fn negative_amount_rejected() {
        assert_eq!(
            PrecipitationAmount::new(-0.1),
            Err(PrecipitationValueError::NegativeAmount(-0.1))
        );
    }

    #[test]
    fn valid_rate() {
        let rate = PrecipitationRate::new(1.8).unwrap();

        assert_eq!(rate.millimeters_per_hour(), 1.8);
    }

    #[test]
    fn negative_rate_rejected() {
        assert_eq!(
            PrecipitationRate::new(-0.1),
            Err(PrecipitationValueError::NegativeRate(-0.1))
        );
    }

    #[test]
    fn intensity_display_and_parse() {
        assert_eq!(PrecipitationIntensity::Heavy.to_string(), "heavy");
        assert_eq!(
            PrecipitationIntensity::from_str("moderate").unwrap(),
            PrecipitationIntensity::Moderate
        );
        assert_eq!(
            PrecipitationIntensity::from_str(" "),
            Err(PrecipitationIntensityParseError::Empty)
        );
    }
}
