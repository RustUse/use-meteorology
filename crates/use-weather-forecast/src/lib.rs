#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(
    value: impl AsRef<str>,
    error: ForecastValueError,
) -> Result<String, ForecastValueError> {
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

/// Errors returned by forecast constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForecastValueError {
    /// Forecast identifiers cannot be empty.
    EmptyForecastId,
    /// Forecast periods cannot be empty.
    EmptyForecastPeriod,
}

impl fmt::Display for ForecastValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyForecastId => formatter.write_str("forecast identifier cannot be empty"),
            Self::EmptyForecastPeriod => formatter.write_str("forecast period cannot be empty"),
        }
    }
}

impl Error for ForecastValueError {}

/// Stable forecast kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ForecastKind {
    /// Nowcast.
    Nowcast,
    /// Short-range forecast.
    ShortRange,
    /// Medium-range forecast.
    MediumRange,
    /// Long-range forecast.
    LongRange,
    /// Seasonal forecast.
    Seasonal,
    /// Climate outlook.
    ClimateOutlook,
    /// Unknown forecast kind.
    Unknown,
    /// Caller-defined forecast kind.
    Custom(String),
}

impl fmt::Display for ForecastKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nowcast => formatter.write_str("nowcast"),
            Self::ShortRange => formatter.write_str("short-range"),
            Self::MediumRange => formatter.write_str("medium-range"),
            Self::LongRange => formatter.write_str("long-range"),
            Self::Seasonal => formatter.write_str("seasonal"),
            Self::ClimateOutlook => formatter.write_str("climate-outlook"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ForecastKind {
    type Err = ForecastKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ForecastKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "nowcast" => Ok(Self::Nowcast),
            "short-range" | "shortrange" => Ok(Self::ShortRange),
            "medium-range" | "mediumrange" => Ok(Self::MediumRange),
            "long-range" | "longrange" => Ok(Self::LongRange),
            "seasonal" => Ok(Self::Seasonal),
            "climate-outlook" | "climateoutlook" => Ok(Self::ClimateOutlook),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing forecast kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForecastKindParseError {
    /// The forecast kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ForecastKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("forecast kind cannot be empty"),
        }
    }
}

impl Error for ForecastKindParseError {}

/// Stable forecast confidence vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ForecastConfidence {
    /// Low confidence.
    Low,
    /// Medium confidence.
    Medium,
    /// High confidence.
    High,
    /// Unknown confidence.
    Unknown,
    /// Caller-defined confidence.
    Custom(String),
}

impl fmt::Display for ForecastConfidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => formatter.write_str("low"),
            Self::Medium => formatter.write_str("medium"),
            Self::High => formatter.write_str("high"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ForecastConfidence {
    type Err = ForecastConfidenceParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ForecastConfidenceParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing forecast confidence fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForecastConfidenceParseError {
    /// The forecast confidence was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ForecastConfidenceParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("forecast confidence cannot be empty"),
        }
    }
}

impl Error for ForecastConfidenceParseError {}

/// A non-empty forecast identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ForecastId(String);

impl ForecastId {
    /// Creates a forecast identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ForecastValueError::EmptyForecastId`] when the identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ForecastValueError> {
        non_empty_text(value, ForecastValueError::EmptyForecastId).map(Self)
    }

    /// Returns the stored identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ForecastId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ForecastId {
    type Err = ForecastValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Forecast horizon stored as hours.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ForecastHorizon(u32);

impl ForecastHorizon {
    /// Creates a forecast horizon from hours.
    #[must_use]
    pub const fn new(hours: u32) -> Self {
        Self(hours)
    }

    /// Returns the stored horizon in hours.
    #[must_use]
    pub const fn hours(&self) -> u32 {
        self.0
    }
}

/// A non-empty descriptive forecast period label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ForecastPeriod(String);

impl ForecastPeriod {
    /// Creates a forecast period label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ForecastValueError::EmptyForecastPeriod`] when the period is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ForecastValueError> {
        non_empty_text(value, ForecastValueError::EmptyForecastPeriod).map(Self)
    }

    /// Returns the stored period label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ForecastPeriod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ForecastPeriod {
    type Err = ForecastValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ForecastConfidence, ForecastConfidenceParseError, ForecastHorizon, ForecastId,
        ForecastKind, ForecastKindParseError, ForecastValueError,
    };
    use core::str::FromStr;

    #[test]
    fn valid_forecast_id() {
        let identifier = ForecastId::new("fcst-001").unwrap();

        assert_eq!(identifier.as_str(), "fcst-001");
    }

    #[test]
    fn empty_forecast_id_rejected() {
        assert_eq!(
            ForecastId::new(" "),
            Err(ForecastValueError::EmptyForecastId)
        );
    }

    #[test]
    fn forecast_kind_display_and_parse() {
        assert_eq!(ForecastKind::ClimateOutlook.to_string(), "climate-outlook");
        assert_eq!(
            ForecastKind::from_str("short range").unwrap(),
            ForecastKind::ShortRange
        );
        assert_eq!(
            ForecastKind::from_str(" "),
            Err(ForecastKindParseError::Empty)
        );
    }

    #[test]
    fn forecast_confidence_display_and_parse() {
        assert_eq!(ForecastConfidence::Medium.to_string(), "medium");
        assert_eq!(
            ForecastConfidence::from_str("high").unwrap(),
            ForecastConfidence::High
        );
        assert_eq!(
            ForecastConfidence::from_str(" "),
            Err(ForecastConfidenceParseError::Empty)
        );
    }

    #[test]
    fn forecast_horizon_construction() {
        let horizon = ForecastHorizon::new(48);

        assert_eq!(horizon.hours(), 48);
    }

    #[test]
    fn custom_forecast_kind() {
        assert_eq!(
            ForecastKind::from_str("convective outlook").unwrap(),
            ForecastKind::Custom(String::from("convective outlook"))
        );
    }
}
