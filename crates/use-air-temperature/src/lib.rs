#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn validate_celsius(value: f64) -> Result<f64, TemperatureValueError> {
    if !value.is_finite() {
        Err(TemperatureValueError::NonFiniteCelsius(value))
    } else {
        Ok(value)
    }
}

/// Error returned when temperature values are not finite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TemperatureValueError {
    /// The supplied Celsius value was `NaN` or infinite.
    NonFiniteCelsius(f64),
}

impl fmt::Display for TemperatureValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteCelsius(value) => {
                write!(formatter, "temperature value must be finite, got {value}")
            },
        }
    }
}

impl Error for TemperatureValueError {}

/// Stable meteorological temperature kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TemperatureKind {
    /// Air temperature.
    Air,
    /// Dew-point temperature.
    DewPoint,
    /// Wet-bulb temperature.
    WetBulb,
    /// Heat index.
    HeatIndex,
    /// Wind chill.
    WindChill,
    /// Apparent temperature.
    Apparent,
    /// Unknown temperature kind.
    Unknown,
    /// Caller-defined temperature kind text.
    Custom(String),
}

impl fmt::Display for TemperatureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Air => formatter.write_str("air"),
            Self::DewPoint => formatter.write_str("dew-point"),
            Self::WetBulb => formatter.write_str("wet-bulb"),
            Self::HeatIndex => formatter.write_str("heat-index"),
            Self::WindChill => formatter.write_str("wind-chill"),
            Self::Apparent => formatter.write_str("apparent"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for TemperatureKind {
    type Err = TemperatureKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(TemperatureKindParseError::Empty);
        }

        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "air" => Ok(Self::Air),
            "dew-point" | "dewpoint" => Ok(Self::DewPoint),
            "wet-bulb" | "wetbulb" => Ok(Self::WetBulb),
            "heat-index" | "heatindex" => Ok(Self::HeatIndex),
            "wind-chill" | "windchill" => Ok(Self::WindChill),
            "apparent" => Ok(Self::Apparent),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing temperature kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TemperatureKindParseError {
    /// The temperature kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for TemperatureKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("temperature kind cannot be empty"),
        }
    }
}

impl Error for TemperatureKindParseError {}

/// Air temperature stored in Celsius.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AirTemperature(f64);

impl AirTemperature {
    /// Creates an air temperature from a finite Celsius value.
    ///
    /// # Errors
    ///
    /// Returns [`TemperatureValueError::NonFiniteCelsius`] when the value is not finite.
    pub fn new(celsius: f64) -> Result<Self, TemperatureValueError> {
        validate_celsius(celsius).map(Self)
    }

    /// Returns the stored Celsius value.
    #[must_use]
    pub fn celsius(&self) -> f64 {
        self.0
    }
}

/// Dew-point temperature stored in Celsius.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct DewPoint(f64);

impl DewPoint {
    /// Creates a dew-point value from a finite Celsius value.
    ///
    /// # Errors
    ///
    /// Returns [`TemperatureValueError::NonFiniteCelsius`] when the value is not finite.
    pub fn new(celsius: f64) -> Result<Self, TemperatureValueError> {
        validate_celsius(celsius).map(Self)
    }

    /// Returns the stored Celsius value.
    #[must_use]
    pub fn celsius(&self) -> f64 {
        self.0
    }
}

/// Heat index stored as a descriptive Celsius value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HeatIndex(f64);

impl HeatIndex {
    /// Creates a heat-index value from a finite Celsius value.
    ///
    /// # Errors
    ///
    /// Returns [`TemperatureValueError::NonFiniteCelsius`] when the value is not finite.
    pub fn new(celsius: f64) -> Result<Self, TemperatureValueError> {
        validate_celsius(celsius).map(Self)
    }

    /// Returns the stored Celsius value.
    #[must_use]
    pub fn celsius(&self) -> f64 {
        self.0
    }
}

/// Wind chill stored as a descriptive Celsius value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct WindChill(f64);

impl WindChill {
    /// Creates a wind-chill value from a finite Celsius value.
    ///
    /// # Errors
    ///
    /// Returns [`TemperatureValueError::NonFiniteCelsius`] when the value is not finite.
    pub fn new(celsius: f64) -> Result<Self, TemperatureValueError> {
        validate_celsius(celsius).map(Self)
    }

    /// Returns the stored Celsius value.
    #[must_use]
    pub fn celsius(&self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AirTemperature, DewPoint, TemperatureKind, TemperatureKindParseError, TemperatureValueError,
    };
    use core::str::FromStr;

    #[test]
    fn valid_positive_temperature() {
        let value = AirTemperature::new(24.5).unwrap();

        assert_eq!(value.celsius(), 24.5);
    }

    #[test]
    fn valid_negative_temperature() {
        let value = AirTemperature::new(-18.0).unwrap();

        assert_eq!(value.celsius(), -18.0);
    }

    #[test]
    fn dew_point_construction() {
        let value = DewPoint::new(9.25).unwrap();

        assert_eq!(value.celsius(), 9.25);
    }

    #[test]
    fn temperature_kind_display_and_parse() {
        assert_eq!(TemperatureKind::WindChill.to_string(), "wind-chill");
        assert_eq!(
            TemperatureKind::from_str("dew point").unwrap(),
            TemperatureKind::DewPoint
        );
        assert_eq!(
            TemperatureKind::from_str(" "),
            Err(TemperatureKindParseError::Empty)
        );
    }

    #[test]
    fn custom_temperature_kind() {
        assert_eq!(
            TemperatureKind::from_str("frost point").unwrap(),
            TemperatureKind::Custom(String::from("frost point"))
        );
    }

    #[test]
    fn rejects_non_finite_temperature() {
        assert_eq!(
            AirTemperature::new(f64::INFINITY),
            Err(TemperatureValueError::NonFiniteCelsius(f64::INFINITY))
        );
    }
}
