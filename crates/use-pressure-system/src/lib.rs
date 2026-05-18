#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(
    value: impl AsRef<str>,
    error: PressureSystemValueError,
) -> Result<String, PressureSystemValueError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Errors returned by pressure-system constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PressureSystemValueError {
    /// Pressure system names cannot be empty.
    EmptyPressureSystemName,
    /// Pressure system strength labels cannot be empty.
    EmptyPressureSystemStrength,
}

impl fmt::Display for PressureSystemValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPressureSystemName => {
                formatter.write_str("pressure system name cannot be empty")
            },
            Self::EmptyPressureSystemStrength => {
                formatter.write_str("pressure system strength cannot be empty")
            },
        }
    }
}

impl Error for PressureSystemValueError {}

/// Stable pressure-system kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PressureSystemKind {
    /// High pressure.
    High,
    /// Low pressure.
    Low,
    /// Ridge.
    Ridge,
    /// Trough.
    Trough,
    /// Cyclone.
    Cyclone,
    /// Anticyclone.
    Anticyclone,
    /// Unknown pressure system kind.
    Unknown,
    /// Caller-defined pressure system kind.
    Custom(String),
}

impl fmt::Display for PressureSystemKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::High => formatter.write_str("high"),
            Self::Low => formatter.write_str("low"),
            Self::Ridge => formatter.write_str("ridge"),
            Self::Trough => formatter.write_str("trough"),
            Self::Cyclone => formatter.write_str("cyclone"),
            Self::Anticyclone => formatter.write_str("anticyclone"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for PressureSystemKind {
    type Err = PressureSystemKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PressureSystemKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            "ridge" => Ok(Self::Ridge),
            "trough" => Ok(Self::Trough),
            "cyclone" => Ok(Self::Cyclone),
            "anticyclone" => Ok(Self::Anticyclone),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing pressure-system kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PressureSystemKindParseError {
    /// The pressure system kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PressureSystemKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pressure system kind cannot be empty"),
        }
    }
}

impl Error for PressureSystemKindParseError {}

/// Stable cyclone-kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CycloneKind {
    /// Tropical cyclone.
    TropicalCyclone,
    /// Extratropical cyclone.
    ExtratropicalCyclone,
    /// Subtropical cyclone.
    SubtropicalCyclone,
    /// Mesocyclone.
    Mesocyclone,
    /// Polar low.
    PolarLow,
    /// Unknown cyclone kind.
    Unknown,
    /// Caller-defined cyclone kind.
    Custom(String),
}

impl fmt::Display for CycloneKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TropicalCyclone => formatter.write_str("tropical-cyclone"),
            Self::ExtratropicalCyclone => formatter.write_str("extratropical-cyclone"),
            Self::SubtropicalCyclone => formatter.write_str("subtropical-cyclone"),
            Self::Mesocyclone => formatter.write_str("mesocyclone"),
            Self::PolarLow => formatter.write_str("polar-low"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for CycloneKind {
    type Err = CycloneKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CycloneKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "tropical-cyclone" | "tropicalcyclone" => Ok(Self::TropicalCyclone),
            "extratropical-cyclone" | "extratropicalcyclone" => Ok(Self::ExtratropicalCyclone),
            "subtropical-cyclone" | "subtropicalcyclone" => Ok(Self::SubtropicalCyclone),
            "mesocyclone" => Ok(Self::Mesocyclone),
            "polar-low" | "polarlow" => Ok(Self::PolarLow),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing cyclone kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CycloneKindParseError {
    /// The cyclone kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for CycloneKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("cyclone kind cannot be empty"),
        }
    }
}

impl Error for CycloneKindParseError {}

/// A non-empty pressure system name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PressureSystemName(String);

impl PressureSystemName {
    /// Creates a pressure system name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PressureSystemValueError::EmptyPressureSystemName`] when the name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PressureSystemValueError> {
        non_empty_text(value, PressureSystemValueError::EmptyPressureSystemName).map(Self)
    }

    /// Returns the stored name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PressureSystemName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PressureSystemName {
    type Err = PressureSystemValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty descriptive pressure system strength label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PressureSystemStrength(String);

impl PressureSystemStrength {
    /// Creates a pressure system strength label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PressureSystemValueError::EmptyPressureSystemStrength`] when the label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PressureSystemValueError> {
        non_empty_text(value, PressureSystemValueError::EmptyPressureSystemStrength).map(Self)
    }

    /// Returns the stored strength label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PressureSystemStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PressureSystemStrength {
    type Err = PressureSystemValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CycloneKind, CycloneKindParseError, PressureSystemKind, PressureSystemKindParseError,
        PressureSystemName, PressureSystemValueError,
    };
    use core::str::FromStr;

    #[test]
    fn valid_pressure_system_name() {
        let name = PressureSystemName::new("Aleutian Low").unwrap();

        assert_eq!(name.as_str(), "Aleutian Low");
    }

    #[test]
    fn empty_pressure_system_name_rejected() {
        assert_eq!(
            PressureSystemName::new(" "),
            Err(PressureSystemValueError::EmptyPressureSystemName)
        );
    }

    #[test]
    fn pressure_system_kind_display_and_parse() {
        assert_eq!(PressureSystemKind::Anticyclone.to_string(), "anticyclone");
        assert_eq!(
            PressureSystemKind::from_str("ridge").unwrap(),
            PressureSystemKind::Ridge
        );
        assert_eq!(
            PressureSystemKind::from_str(" "),
            Err(PressureSystemKindParseError::Empty)
        );
    }

    #[test]
    fn cyclone_kind_display_and_parse() {
        assert_eq!(CycloneKind::PolarLow.to_string(), "polar-low");
        assert_eq!(
            CycloneKind::from_str("mesocyclone").unwrap(),
            CycloneKind::Mesocyclone
        );
        assert_eq!(
            CycloneKind::from_str(" "),
            Err(CycloneKindParseError::Empty)
        );
    }

    #[test]
    fn custom_pressure_system_kind() {
        assert_eq!(
            PressureSystemKind::from_str("blocking high").unwrap(),
            PressureSystemKind::Custom(String::from("blocking high"))
        );
    }
}
