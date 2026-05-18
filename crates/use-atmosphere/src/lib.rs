#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AtmosphericConditionError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(AtmosphericConditionError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned when atmospheric condition text is empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AtmosphericConditionError {
    /// The supplied condition label was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for AtmosphericConditionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("atmospheric condition cannot be empty"),
        }
    }
}

impl Error for AtmosphericConditionError {}

/// Stable atmosphere layer vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AtmosphereLayer {
    /// Troposphere.
    Troposphere,
    /// Stratosphere.
    Stratosphere,
    /// Mesosphere.
    Mesosphere,
    /// Thermosphere.
    Thermosphere,
    /// Exosphere.
    Exosphere,
    /// Unknown atmosphere layer.
    Unknown,
    /// Caller-defined atmosphere layer.
    Custom(String),
}

impl fmt::Display for AtmosphereLayer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Troposphere => formatter.write_str("troposphere"),
            Self::Stratosphere => formatter.write_str("stratosphere"),
            Self::Mesosphere => formatter.write_str("mesosphere"),
            Self::Thermosphere => formatter.write_str("thermosphere"),
            Self::Exosphere => formatter.write_str("exosphere"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for AtmosphereLayer {
    type Err = AtmosphereLayerParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(AtmosphereLayerParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "troposphere" => Ok(Self::Troposphere),
            "stratosphere" => Ok(Self::Stratosphere),
            "mesosphere" => Ok(Self::Mesosphere),
            "thermosphere" => Ok(Self::Thermosphere),
            "exosphere" => Ok(Self::Exosphere),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing atmosphere layers fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AtmosphereLayerParseError {
    /// The atmosphere layer was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for AtmosphereLayerParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("atmosphere layer cannot be empty"),
        }
    }
}

impl Error for AtmosphereLayerParseError {}

/// Stable air-mass vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AirMassKind {
    /// Continental polar.
    ContinentalPolar,
    /// Continental tropical.
    ContinentalTropical,
    /// Maritime polar.
    MaritimePolar,
    /// Maritime tropical.
    MaritimeTropical,
    /// Arctic.
    Arctic,
    /// Antarctic.
    Antarctic,
    /// Unknown air mass.
    Unknown,
    /// Caller-defined air-mass label.
    Custom(String),
}

impl fmt::Display for AirMassKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContinentalPolar => formatter.write_str("continental-polar"),
            Self::ContinentalTropical => formatter.write_str("continental-tropical"),
            Self::MaritimePolar => formatter.write_str("maritime-polar"),
            Self::MaritimeTropical => formatter.write_str("maritime-tropical"),
            Self::Arctic => formatter.write_str("arctic"),
            Self::Antarctic => formatter.write_str("antarctic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for AirMassKind {
    type Err = AirMassKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(AirMassKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "continental-polar" => Ok(Self::ContinentalPolar),
            "continental-tropical" => Ok(Self::ContinentalTropical),
            "maritime-polar" => Ok(Self::MaritimePolar),
            "maritime-tropical" => Ok(Self::MaritimeTropical),
            "arctic" => Ok(Self::Arctic),
            "antarctic" => Ok(Self::Antarctic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing air masses fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AirMassKindParseError {
    /// The air mass kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for AirMassKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("air mass kind cannot be empty"),
        }
    }
}

impl Error for AirMassKindParseError {}

/// Stable visibility condition vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VisibilityCondition {
    /// Clear visibility.
    Clear,
    /// Haze.
    Haze,
    /// Mist.
    Mist,
    /// Fog.
    Fog,
    /// Smoke.
    Smoke,
    /// Dust.
    Dust,
    /// Unknown visibility condition.
    Unknown,
    /// Caller-defined visibility condition.
    Custom(String),
}

impl fmt::Display for VisibilityCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clear => formatter.write_str("clear"),
            Self::Haze => formatter.write_str("haze"),
            Self::Mist => formatter.write_str("mist"),
            Self::Fog => formatter.write_str("fog"),
            Self::Smoke => formatter.write_str("smoke"),
            Self::Dust => formatter.write_str("dust"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for VisibilityCondition {
    type Err = VisibilityConditionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(VisibilityConditionParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "clear" => Ok(Self::Clear),
            "haze" => Ok(Self::Haze),
            "mist" => Ok(Self::Mist),
            "fog" => Ok(Self::Fog),
            "smoke" => Ok(Self::Smoke),
            "dust" => Ok(Self::Dust),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing visibility conditions fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisibilityConditionParseError {
    /// The visibility condition was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for VisibilityConditionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("visibility condition cannot be empty"),
        }
    }
}

impl Error for VisibilityConditionParseError {}

/// A descriptive non-empty atmospheric condition label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AtmosphericCondition(String);

impl AtmosphericCondition {
    /// Creates an atmospheric condition label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`AtmosphericConditionError::Empty`] when the label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AtmosphericConditionError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the stored atmospheric condition text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the condition and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for AtmosphericCondition {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AtmosphericCondition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AtmosphericCondition {
    type Err = AtmosphericConditionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AirMassKind, AirMassKindParseError, AtmosphereLayer, AtmosphereLayerParseError,
        AtmosphericCondition, AtmosphericConditionError, VisibilityCondition,
        VisibilityConditionParseError,
    };
    use core::str::FromStr;

    #[test]
    fn atmosphere_layer_display_and_parse() {
        assert_eq!(AtmosphereLayer::Stratosphere.to_string(), "stratosphere");
        assert_eq!(
            AtmosphereLayer::from_str("mesosphere").unwrap(),
            AtmosphereLayer::Mesosphere
        );
        assert_eq!(
            AtmosphereLayer::from_str(" "),
            Err(AtmosphereLayerParseError::Empty)
        );
    }

    #[test]
    fn air_mass_display_and_parse() {
        assert_eq!(AirMassKind::MaritimePolar.to_string(), "maritime-polar");
        assert_eq!(
            AirMassKind::from_str("continental tropical").unwrap(),
            AirMassKind::ContinentalTropical
        );
        assert_eq!(
            AirMassKind::from_str(" "),
            Err(AirMassKindParseError::Empty)
        );
    }

    #[test]
    fn visibility_condition_display_and_parse() {
        assert_eq!(VisibilityCondition::Fog.to_string(), "fog");
        assert_eq!(
            VisibilityCondition::from_str("smoke").unwrap(),
            VisibilityCondition::Smoke
        );
        assert_eq!(
            VisibilityCondition::from_str(" "),
            Err(VisibilityConditionParseError::Empty)
        );
    }

    #[test]
    fn custom_atmosphere_layer() {
        assert_eq!(
            AtmosphereLayer::from_str("planetary boundary layer").unwrap(),
            AtmosphereLayer::Custom(String::from("planetary boundary layer"))
        );
    }

    #[test]
    fn atmospheric_condition_requires_text() {
        assert_eq!(
            AtmosphericCondition::new("   "),
            Err(AtmosphericConditionError::Empty)
        );
        assert_eq!(
            AtmosphericCondition::new("unstable low levels")
                .unwrap()
                .as_str(),
            "unstable low levels"
        );
    }
}
