#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Stable weather-front kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WeatherFrontKind {
    /// Cold front.
    Cold,
    /// Warm front.
    Warm,
    /// Stationary front.
    Stationary,
    /// Occluded front.
    Occluded,
    /// Dryline.
    Dryline,
    /// Squall line.
    SquallLine,
    /// Unknown front kind.
    Unknown,
    /// Caller-defined front kind.
    Custom(String),
}

impl fmt::Display for WeatherFrontKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cold => formatter.write_str("cold"),
            Self::Warm => formatter.write_str("warm"),
            Self::Stationary => formatter.write_str("stationary"),
            Self::Occluded => formatter.write_str("occluded"),
            Self::Dryline => formatter.write_str("dryline"),
            Self::SquallLine => formatter.write_str("squall-line"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for WeatherFrontKind {
    type Err = WeatherFrontKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(WeatherFrontKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "cold" => Ok(Self::Cold),
            "warm" => Ok(Self::Warm),
            "stationary" => Ok(Self::Stationary),
            "occluded" => Ok(Self::Occluded),
            "dryline" => Ok(Self::Dryline),
            "squall-line" | "squallline" => Ok(Self::SquallLine),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing weather-front kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherFrontKindParseError {
    /// The front kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for WeatherFrontKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("weather front kind cannot be empty"),
        }
    }
}

impl Error for WeatherFrontKindParseError {}

/// Stable weather-front movement vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FrontMovement {
    /// Advancing front.
    Advancing,
    /// Retreating front.
    Retreating,
    /// Stationary front.
    Stationary,
    /// Weakening front.
    Weakening,
    /// Strengthening front.
    Strengthening,
    /// Unknown front movement.
    Unknown,
    /// Caller-defined movement.
    Custom(String),
}

impl fmt::Display for FrontMovement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Advancing => formatter.write_str("advancing"),
            Self::Retreating => formatter.write_str("retreating"),
            Self::Stationary => formatter.write_str("stationary"),
            Self::Weakening => formatter.write_str("weakening"),
            Self::Strengthening => formatter.write_str("strengthening"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FrontMovement {
    type Err = FrontMovementParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FrontMovementParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "advancing" => Ok(Self::Advancing),
            "retreating" => Ok(Self::Retreating),
            "stationary" => Ok(Self::Stationary),
            "weakening" => Ok(Self::Weakening),
            "strengthening" => Ok(Self::Strengthening),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing front movement fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontMovementParseError {
    /// The front movement was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for FrontMovementParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("front movement cannot be empty"),
        }
    }
}

impl Error for FrontMovementParseError {}

/// Stable weather-front strength vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FrontStrength {
    /// Weak front.
    Weak,
    /// Moderate front.
    Moderate,
    /// Strong front.
    Strong,
    /// Severe front.
    Severe,
    /// Unknown front strength.
    Unknown,
    /// Caller-defined strength.
    Custom(String),
}

impl fmt::Display for FrontStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weak => formatter.write_str("weak"),
            Self::Moderate => formatter.write_str("moderate"),
            Self::Strong => formatter.write_str("strong"),
            Self::Severe => formatter.write_str("severe"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FrontStrength {
    type Err = FrontStrengthParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FrontStrengthParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "weak" => Ok(Self::Weak),
            "moderate" => Ok(Self::Moderate),
            "strong" => Ok(Self::Strong),
            "severe" => Ok(Self::Severe),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing front strength fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontStrengthParseError {
    /// The front strength was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for FrontStrengthParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("front strength cannot be empty"),
        }
    }
}

impl Error for FrontStrengthParseError {}

#[cfg(test)]
mod tests {
    use super::{
        FrontMovement, FrontMovementParseError, FrontStrength, FrontStrengthParseError,
        WeatherFrontKind, WeatherFrontKindParseError,
    };
    use core::str::FromStr;

    #[test]
    fn front_kind_display_and_parse() {
        assert_eq!(WeatherFrontKind::SquallLine.to_string(), "squall-line");
        assert_eq!(
            WeatherFrontKind::from_str("cold").unwrap(),
            WeatherFrontKind::Cold
        );
        assert_eq!(
            WeatherFrontKind::from_str(" "),
            Err(WeatherFrontKindParseError::Empty)
        );
    }

    #[test]
    fn movement_display_and_parse() {
        assert_eq!(FrontMovement::Advancing.to_string(), "advancing");
        assert_eq!(
            FrontMovement::from_str("weakening").unwrap(),
            FrontMovement::Weakening
        );
        assert_eq!(
            FrontMovement::from_str(" "),
            Err(FrontMovementParseError::Empty)
        );
    }

    #[test]
    fn strength_display_and_parse() {
        assert_eq!(FrontStrength::Strong.to_string(), "strong");
        assert_eq!(
            FrontStrength::from_str("severe").unwrap(),
            FrontStrength::Severe
        );
        assert_eq!(
            FrontStrength::from_str(" "),
            Err(FrontStrengthParseError::Empty)
        );
    }

    #[test]
    fn custom_front_kind() {
        assert_eq!(
            WeatherFrontKind::from_str("lee trough").unwrap(),
            WeatherFrontKind::Custom(String::from("lee trough"))
        );
    }
}
