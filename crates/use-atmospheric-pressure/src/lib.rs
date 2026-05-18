#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn validate_pressure(value: f64) -> Result<f64, PressureValueError> {
    if !value.is_finite() {
        return Err(PressureValueError::NonFinitePressure(value));
    }

    if value < 0.0 {
        return Err(PressureValueError::NegativePressure(value));
    }

    Ok(value)
}

/// Errors returned by pressure value constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PressureValueError {
    /// The supplied pressure was `NaN` or infinite.
    NonFinitePressure(f64),
    /// The supplied pressure was negative.
    NegativePressure(f64),
}

impl fmt::Display for PressureValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinitePressure(value) => {
                write!(formatter, "pressure must be finite, got {value}")
            },
            Self::NegativePressure(value) => {
                write!(formatter, "pressure cannot be negative, got {value}")
            },
        }
    }
}

impl Error for PressureValueError {}

/// Simple atmospheric pressure unit labels for the stored `f64` values.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PressureUnitLabel {
    /// Hectopascals.
    Hectopascal,
    /// Millibars.
    Millibar,
    /// Unknown unit label.
    Unknown,
    /// Caller-defined unit label.
    Custom(String),
}

impl fmt::Display for PressureUnitLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hectopascal => formatter.write_str("hPa"),
            Self::Millibar => formatter.write_str("mbar"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PressureUnitLabel {
    type Err = PressureUnitLabelParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PressureUnitLabelParseError::Empty);
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "hpa" | "hectopascal" | "hectopascals" => Ok(Self::Hectopascal),
            "mbar" | "millibar" | "millibars" => Ok(Self::Millibar),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing pressure unit labels fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PressureUnitLabelParseError {
    /// The pressure unit label was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PressureUnitLabelParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pressure unit label cannot be empty"),
        }
    }
}

impl Error for PressureUnitLabelParseError {}

/// Stable atmospheric pressure tendency vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PressureTendency {
    /// Rising pressure.
    Rising,
    /// Falling pressure.
    Falling,
    /// Steady pressure.
    Steady,
    /// Unknown tendency.
    Unknown,
    /// Caller-defined tendency text.
    Custom(String),
}

impl fmt::Display for PressureTendency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rising => formatter.write_str("rising"),
            Self::Falling => formatter.write_str("falling"),
            Self::Steady => formatter.write_str("steady"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PressureTendency {
    type Err = PressureTendencyParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PressureTendencyParseError::Empty);
        }

        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "rising" => Ok(Self::Rising),
            "falling" => Ok(Self::Falling),
            "steady" => Ok(Self::Steady),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing pressure tendencies fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PressureTendencyParseError {
    /// The pressure tendency was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PressureTendencyParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pressure tendency cannot be empty"),
        }
    }
}

impl Error for PressureTendencyParseError {}

/// Atmospheric pressure stored in hPa or mbar.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AtmosphericPressure(f64);

impl AtmosphericPressure {
    /// Creates atmospheric pressure from a finite non-negative hPa value.
    ///
    /// # Errors
    ///
    /// Returns [`PressureValueError`] when the pressure is invalid.
    pub fn new(hectopascals: f64) -> Result<Self, PressureValueError> {
        validate_pressure(hectopascals).map(Self)
    }

    /// Returns the stored pressure in hPa.
    #[must_use]
    pub fn hectopascals(&self) -> f64 {
        self.0
    }

    /// Returns the unit label for the stored convention.
    #[must_use]
    pub fn unit_label(&self) -> PressureUnitLabel {
        PressureUnitLabel::Hectopascal
    }
}

/// Sea-level pressure stored in hPa or mbar.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SeaLevelPressure(f64);

impl SeaLevelPressure {
    /// Creates sea-level pressure from a finite non-negative hPa value.
    ///
    /// # Errors
    ///
    /// Returns [`PressureValueError`] when the pressure is invalid.
    pub fn new(hectopascals: f64) -> Result<Self, PressureValueError> {
        validate_pressure(hectopascals).map(Self)
    }

    /// Returns the stored pressure in hPa.
    #[must_use]
    pub fn hectopascals(&self) -> f64 {
        self.0
    }

    /// Returns the unit label for the stored convention.
    #[must_use]
    pub fn unit_label(&self) -> PressureUnitLabel {
        PressureUnitLabel::Hectopascal
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AtmosphericPressure, PressureTendency, PressureTendencyParseError, PressureValueError,
        SeaLevelPressure,
    };
    use core::str::FromStr;

    #[test]
    fn valid_pressure() {
        let pressure = AtmosphericPressure::new(1008.5).unwrap();

        assert_eq!(pressure.hectopascals(), 1008.5);
    }

    #[test]
    fn negative_pressure_rejected() {
        assert_eq!(
            AtmosphericPressure::new(-1.0),
            Err(PressureValueError::NegativePressure(-1.0))
        );
    }

    #[test]
    fn sea_level_pressure_construction() {
        let pressure = SeaLevelPressure::new(1016.3).unwrap();

        assert_eq!(pressure.hectopascals(), 1016.3);
    }

    #[test]
    fn tendency_display_and_parse() {
        assert_eq!(PressureTendency::Steady.to_string(), "steady");
        assert_eq!(
            PressureTendency::from_str("falling").unwrap(),
            PressureTendency::Falling
        );
        assert_eq!(
            PressureTendency::from_str(" "),
            Err(PressureTendencyParseError::Empty)
        );
    }

    #[test]
    fn custom_tendency() {
        assert_eq!(
            PressureTendency::from_str("rapid rise").unwrap(),
            PressureTendency::Custom(String::from("rapid rise"))
        );
    }
}
