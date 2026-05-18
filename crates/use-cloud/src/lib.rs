#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Errors returned by cloud constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CloudValueError {
    /// Cloud cover must stay in `0..=8` oktas.
    CloudCoverOutOfRange(u8),
    /// Cloud base must be finite.
    NonFiniteCloudBase(f64),
    /// Cloud base cannot be negative.
    NegativeCloudBase(f64),
}

impl fmt::Display for CloudValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CloudCoverOutOfRange(value) => {
                write!(formatter, "cloud cover must be in 0..=8 oktas, got {value}")
            },
            Self::NonFiniteCloudBase(value) => {
                write!(formatter, "cloud base must be finite, got {value}")
            },
            Self::NegativeCloudBase(value) => {
                write!(formatter, "cloud base cannot be negative, got {value}")
            },
        }
    }
}

impl Error for CloudValueError {}

/// Stable cloud kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CloudKind {
    /// Cirrus clouds.
    Cirrus,
    /// Cirrostratus clouds.
    Cirrostratus,
    /// Cirrocumulus clouds.
    Cirrocumulus,
    /// Altostratus clouds.
    Altostratus,
    /// Altocumulus clouds.
    Altocumulus,
    /// Stratus clouds.
    Stratus,
    /// Stratocumulus clouds.
    Stratocumulus,
    /// Cumulus clouds.
    Cumulus,
    /// Cumulonimbus clouds.
    Cumulonimbus,
    /// Nimbostratus clouds.
    Nimbostratus,
    /// Fog.
    Fog,
    /// Unknown cloud kind.
    Unknown,
    /// Caller-defined cloud kind.
    Custom(String),
}

impl fmt::Display for CloudKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cirrus => formatter.write_str("cirrus"),
            Self::Cirrostratus => formatter.write_str("cirrostratus"),
            Self::Cirrocumulus => formatter.write_str("cirrocumulus"),
            Self::Altostratus => formatter.write_str("altostratus"),
            Self::Altocumulus => formatter.write_str("altocumulus"),
            Self::Stratus => formatter.write_str("stratus"),
            Self::Stratocumulus => formatter.write_str("stratocumulus"),
            Self::Cumulus => formatter.write_str("cumulus"),
            Self::Cumulonimbus => formatter.write_str("cumulonimbus"),
            Self::Nimbostratus => formatter.write_str("nimbostratus"),
            Self::Fog => formatter.write_str("fog"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for CloudKind {
    type Err = CloudKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CloudKindParseError::Empty);
        }

        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "cirrus" => Ok(Self::Cirrus),
            "cirrostratus" => Ok(Self::Cirrostratus),
            "cirrocumulus" => Ok(Self::Cirrocumulus),
            "altostratus" => Ok(Self::Altostratus),
            "altocumulus" => Ok(Self::Altocumulus),
            "stratus" => Ok(Self::Stratus),
            "stratocumulus" => Ok(Self::Stratocumulus),
            "cumulus" => Ok(Self::Cumulus),
            "cumulonimbus" => Ok(Self::Cumulonimbus),
            "nimbostratus" => Ok(Self::Nimbostratus),
            "fog" => Ok(Self::Fog),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing cloud kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CloudKindParseError {
    /// The cloud kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for CloudKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("cloud kind cannot be empty"),
        }
    }
}

impl Error for CloudKindParseError {}

/// Cloud cover stored in oktas.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CloudCover(u8);

impl CloudCover {
    /// Creates cloud cover from oktas in `0..=8`.
    ///
    /// # Errors
    ///
    /// Returns [`CloudValueError::CloudCoverOutOfRange`] when the value exceeds `8`.
    pub fn new(oktas: u8) -> Result<Self, CloudValueError> {
        if oktas > 8 {
            Err(CloudValueError::CloudCoverOutOfRange(oktas))
        } else {
            Ok(Self(oktas))
        }
    }

    /// Returns the stored cloud cover in oktas.
    #[must_use]
    pub fn oktas(&self) -> u8 {
        self.0
    }
}

/// Cloud base stored in meters above ground level.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CloudBase(f64);

impl CloudBase {
    /// Creates cloud base from a finite non-negative meters-above-ground value.
    ///
    /// # Errors
    ///
    /// Returns [`CloudValueError`] when the base is invalid.
    pub fn new(meters_agl: f64) -> Result<Self, CloudValueError> {
        if !meters_agl.is_finite() {
            return Err(CloudValueError::NonFiniteCloudBase(meters_agl));
        }

        if meters_agl < 0.0 {
            return Err(CloudValueError::NegativeCloudBase(meters_agl));
        }

        Ok(Self(meters_agl))
    }

    /// Returns the stored base in meters above ground level.
    #[must_use]
    pub fn meters_agl(&self) -> f64 {
        self.0
    }
}

/// A descriptive cloud layer built from kind, cover, and optional base.
#[derive(Clone, Debug, PartialEq)]
pub struct CloudLayer {
    kind: CloudKind,
    cover: CloudCover,
    base: Option<CloudBase>,
}

impl CloudLayer {
    /// Creates a cloud layer from cloud kind and cover.
    #[must_use]
    pub fn new(kind: CloudKind, cover: CloudCover) -> Self {
        Self {
            kind,
            cover,
            base: None,
        }
    }

    /// Adds a cloud base to the layer.
    #[must_use]
    pub fn with_base(mut self, base: CloudBase) -> Self {
        self.base = Some(base);
        self
    }

    /// Returns the cloud kind.
    #[must_use]
    pub fn kind(&self) -> &CloudKind {
        &self.kind
    }

    /// Returns the cloud cover.
    #[must_use]
    pub fn cover(&self) -> CloudCover {
        self.cover
    }

    /// Returns the cloud base, if present.
    #[must_use]
    pub fn base(&self) -> Option<CloudBase> {
        self.base
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CloudBase, CloudCover, CloudKind, CloudKindParseError, CloudLayer, CloudValueError,
    };
    use core::str::FromStr;

    #[test]
    fn cloud_kind_display_and_parse() {
        assert_eq!(CloudKind::Cumulonimbus.to_string(), "cumulonimbus");
        assert_eq!(
            CloudKind::from_str("altostratus").unwrap(),
            CloudKind::Altostratus
        );
        assert_eq!(CloudKind::from_str(" "), Err(CloudKindParseError::Empty));
    }

    #[test]
    fn custom_cloud_kind() {
        assert_eq!(
            CloudKind::from_str("lenticular").unwrap(),
            CloudKind::Custom(String::from("lenticular"))
        );
    }

    #[test]
    fn valid_cloud_cover() {
        let cover = CloudCover::new(4).unwrap();

        assert_eq!(cover.oktas(), 4);
    }

    #[test]
    fn invalid_cloud_cover_rejected() {
        assert_eq!(
            CloudCover::new(9),
            Err(CloudValueError::CloudCoverOutOfRange(9))
        );
    }

    #[test]
    fn valid_cloud_base() {
        let base = CloudBase::new(1200.0).unwrap();

        assert_eq!(base.meters_agl(), 1200.0);
    }

    #[test]
    fn negative_cloud_base_rejected() {
        assert_eq!(
            CloudBase::new(-10.0),
            Err(CloudValueError::NegativeCloudBase(-10.0))
        );
    }

    #[test]
    fn cloud_layer_composes_values() {
        let layer = CloudLayer::new(CloudKind::Cumulus, CloudCover::new(3).unwrap())
            .with_base(CloudBase::new(850.0).unwrap());

        assert_eq!(layer.kind(), &CloudKind::Cumulus);
        assert_eq!(layer.cover().oktas(), 3);
        assert_eq!(layer.base().unwrap().meters_agl(), 850.0);
    }
}
