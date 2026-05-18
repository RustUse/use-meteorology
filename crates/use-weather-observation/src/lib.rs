#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(
    value: impl AsRef<str>,
    error: WeatherObservationError,
) -> Result<String, WeatherObservationError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Errors returned by weather observation constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherObservationError {
    /// The observation identifier was empty after trimming whitespace.
    EmptyObservationId,
    /// The observation source text was empty after trimming whitespace.
    EmptyObservationSource,
}

impl fmt::Display for WeatherObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservationId => {
                formatter.write_str("weather observation identifier cannot be empty")
            },
            Self::EmptyObservationSource => {
                formatter.write_str("weather observation source cannot be empty")
            },
        }
    }
}

impl Error for WeatherObservationError {}

/// A non-empty weather observation identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WeatherObservationId(String);

impl WeatherObservationId {
    /// Creates a weather observation identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`WeatherObservationError::EmptyObservationId`] when the identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, WeatherObservationError> {
        non_empty_text(value, WeatherObservationError::EmptyObservationId).map(Self)
    }

    /// Returns the stored identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for WeatherObservationId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for WeatherObservationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WeatherObservationId {
    type Err = WeatherObservationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive non-empty observation source label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObservationSource(String);

impl ObservationSource {
    /// Creates an observation source label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`WeatherObservationError::EmptyObservationSource`] when the source is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, WeatherObservationError> {
        non_empty_text(value, WeatherObservationError::EmptyObservationSource).map(Self)
    }

    /// Returns the stored source label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the source and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ObservationSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ObservationSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ObservationSource {
    type Err = WeatherObservationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Stable weather observation kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservationKind {
    /// Surface observation.
    Surface,
    /// Upper-air observation.
    UpperAir,
    /// Radar observation.
    Radar,
    /// Satellite observation.
    Satellite,
    /// Buoy observation.
    Buoy,
    /// Station observation.
    Station,
    /// Manual observation.
    Manual,
    /// Automated observation.
    Automated,
    /// Unknown observation kind.
    Unknown,
    /// Caller-defined observation kind text.
    Custom(String),
}

impl fmt::Display for ObservationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => formatter.write_str("surface"),
            Self::UpperAir => formatter.write_str("upper-air"),
            Self::Radar => formatter.write_str("radar"),
            Self::Satellite => formatter.write_str("satellite"),
            Self::Buoy => formatter.write_str("buoy"),
            Self::Station => formatter.write_str("station"),
            Self::Manual => formatter.write_str("manual"),
            Self::Automated => formatter.write_str("automated"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ObservationKind {
    type Err = ObservationKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ObservationKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "surface" => Ok(Self::Surface),
            "upper-air" | "upperair" => Ok(Self::UpperAir),
            "radar" => Ok(Self::Radar),
            "satellite" => Ok(Self::Satellite),
            "buoy" => Ok(Self::Buoy),
            "station" => Ok(Self::Station),
            "manual" => Ok(Self::Manual),
            "automated" => Ok(Self::Automated),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing observation kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationKindParseError {
    /// The observation kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ObservationKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("observation kind cannot be empty"),
        }
    }
}

impl Error for ObservationKindParseError {}

/// Stable weather observation quality vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ObservationQuality {
    /// Raw observation.
    Raw,
    /// Estimated observation.
    Estimated,
    /// Corrected observation.
    Corrected,
    /// Verified observation.
    Verified,
    /// Questionable observation.
    Questionable,
    /// Missing observation.
    Missing,
    /// Unknown observation quality.
    Unknown,
    /// Caller-defined observation quality text.
    Custom(String),
}

impl fmt::Display for ObservationQuality {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Raw => formatter.write_str("raw"),
            Self::Estimated => formatter.write_str("estimated"),
            Self::Corrected => formatter.write_str("corrected"),
            Self::Verified => formatter.write_str("verified"),
            Self::Questionable => formatter.write_str("questionable"),
            Self::Missing => formatter.write_str("missing"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ObservationQuality {
    type Err = ObservationQualityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ObservationQualityParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "raw" => Ok(Self::Raw),
            "estimated" => Ok(Self::Estimated),
            "corrected" => Ok(Self::Corrected),
            "verified" => Ok(Self::Verified),
            "questionable" => Ok(Self::Questionable),
            "missing" => Ok(Self::Missing),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing observation quality fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationQualityParseError {
    /// The observation quality was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ObservationQualityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("observation quality cannot be empty"),
        }
    }
}

impl Error for ObservationQualityParseError {}

/// Descriptive weather observation metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeatherObservation {
    id: WeatherObservationId,
    kind: ObservationKind,
    source: ObservationSource,
    quality: ObservationQuality,
}

impl WeatherObservation {
    /// Creates descriptive weather observation metadata.
    #[must_use]
    pub fn new(
        id: WeatherObservationId,
        kind: ObservationKind,
        source: ObservationSource,
        quality: ObservationQuality,
    ) -> Self {
        Self {
            id,
            kind,
            source,
            quality,
        }
    }

    /// Returns the observation identifier.
    #[must_use]
    pub fn id(&self) -> &WeatherObservationId {
        &self.id
    }

    /// Returns the observation kind.
    #[must_use]
    pub fn kind(&self) -> &ObservationKind {
        &self.kind
    }

    /// Returns the descriptive observation source.
    #[must_use]
    pub fn source(&self) -> &ObservationSource {
        &self.source
    }

    /// Returns the observation quality.
    #[must_use]
    pub fn quality(&self) -> &ObservationQuality {
        &self.quality
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ObservationKind, ObservationKindParseError, ObservationQuality,
        ObservationQualityParseError, ObservationSource, WeatherObservation,
        WeatherObservationError, WeatherObservationId,
    };
    use core::str::FromStr;

    #[test]
    fn valid_observation_id() {
        let identifier = WeatherObservationId::new(" obs-001 ").unwrap();

        assert_eq!(identifier.as_str(), "obs-001");
    }

    #[test]
    fn empty_observation_id_rejected() {
        assert_eq!(
            WeatherObservationId::new("   "),
            Err(WeatherObservationError::EmptyObservationId)
        );
    }

    #[test]
    fn observation_kind_display_and_parse() {
        assert_eq!(ObservationKind::UpperAir.to_string(), "upper-air");
        assert_eq!(
            ObservationKind::from_str("upper air").unwrap(),
            ObservationKind::UpperAir
        );
        assert_eq!(
            ObservationKind::from_str(" "),
            Err(ObservationKindParseError::Empty)
        );
    }

    #[test]
    fn observation_quality_display_and_parse() {
        assert_eq!(ObservationQuality::Questionable.to_string(), "questionable");
        assert_eq!(
            ObservationQuality::from_str("verified").unwrap(),
            ObservationQuality::Verified
        );
        assert_eq!(
            ObservationQuality::from_str(" "),
            Err(ObservationQualityParseError::Empty)
        );
    }

    #[test]
    fn custom_observation_kind() {
        assert_eq!(
            ObservationKind::from_str("pilot report").unwrap(),
            ObservationKind::Custom(String::from("pilot report"))
        );
    }

    #[test]
    fn constructs_weather_observation_metadata() {
        let observation = WeatherObservation::new(
            WeatherObservationId::new("obs-002").unwrap(),
            ObservationKind::Radar,
            ObservationSource::new("regional radar composite").unwrap(),
            ObservationQuality::Estimated,
        );

        assert_eq!(observation.id().as_str(), "obs-002");
        assert_eq!(observation.kind(), &ObservationKind::Radar);
        assert_eq!(observation.source().as_str(), "regional radar composite");
        assert_eq!(observation.quality(), &ObservationQuality::Estimated);
    }
}
