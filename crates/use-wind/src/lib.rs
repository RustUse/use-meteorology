#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Errors returned by wind constructors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindValueError {
    /// Wind speed or gust must be finite.
    NonFiniteSpeed(f64),
    /// Wind speed or gust cannot be negative.
    NegativeSpeed(f64),
    /// Wind direction must be finite.
    NonFiniteDirection(f64),
    /// Wind direction must stay in `0.0..360.0`.
    DirectionOutOfRange(f64),
    /// Beaufort scale must stay in `0..=12`.
    BeaufortOutOfRange(u8),
}

impl fmt::Display for WindValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteSpeed(value) => {
                write!(formatter, "wind speed must be finite, got {value}")
            },
            Self::NegativeSpeed(value) => {
                write!(formatter, "wind speed cannot be negative, got {value}")
            },
            Self::NonFiniteDirection(value) => {
                write!(formatter, "wind direction must be finite, got {value}")
            },
            Self::DirectionOutOfRange(value) => {
                write!(
                    formatter,
                    "wind direction must be in 0.0..360.0, got {value}"
                )
            },
            Self::BeaufortOutOfRange(value) => {
                write!(formatter, "Beaufort scale must be in 0..=12, got {value}")
            },
        }
    }
}

impl Error for WindValueError {}

fn validate_speed(value: f64) -> Result<f64, WindValueError> {
    if !value.is_finite() {
        return Err(WindValueError::NonFiniteSpeed(value));
    }

    if value < 0.0 {
        return Err(WindValueError::NegativeSpeed(value));
    }

    Ok(value)
}

/// Stable wind kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WindKind {
    /// Calm conditions.
    Calm,
    /// Breeze conditions.
    Breeze,
    /// Gale conditions.
    Gale,
    /// Storm conditions.
    Storm,
    /// Squall conditions.
    Squall,
    /// Gust conditions.
    Gust,
    /// Unknown wind kind.
    Unknown,
    /// Caller-defined wind kind.
    Custom(String),
}

impl fmt::Display for WindKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Calm => formatter.write_str("calm"),
            Self::Breeze => formatter.write_str("breeze"),
            Self::Gale => formatter.write_str("gale"),
            Self::Storm => formatter.write_str("storm"),
            Self::Squall => formatter.write_str("squall"),
            Self::Gust => formatter.write_str("gust"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for WindKind {
    type Err = WindKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(WindKindParseError::Empty);
        }

        match trimmed
            .to_ascii_lowercase()
            .replace(['_', ' '], "-")
            .as_str()
        {
            "calm" => Ok(Self::Calm),
            "breeze" => Ok(Self::Breeze),
            "gale" => Ok(Self::Gale),
            "storm" => Ok(Self::Storm),
            "squall" => Ok(Self::Squall),
            "gust" => Ok(Self::Gust),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing wind kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindKindParseError {
    /// The wind kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for WindKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("wind kind cannot be empty"),
        }
    }
}

impl Error for WindKindParseError {}

/// Wind speed stored in meters per second.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct WindSpeed(f64);

impl WindSpeed {
    /// Creates wind speed from a finite non-negative value.
    ///
    /// # Errors
    ///
    /// Returns [`WindValueError`] when the speed is invalid.
    pub fn new(meters_per_second: f64) -> Result<Self, WindValueError> {
        validate_speed(meters_per_second).map(Self)
    }

    /// Returns the stored speed in meters per second.
    #[must_use]
    pub fn meters_per_second(&self) -> f64 {
        self.0
    }
}

/// Wind gust stored in meters per second.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct WindGust(f64);

impl WindGust {
    /// Creates wind gust from a finite non-negative value.
    ///
    /// # Errors
    ///
    /// Returns [`WindValueError`] when the gust is invalid.
    pub fn new(meters_per_second: f64) -> Result<Self, WindValueError> {
        validate_speed(meters_per_second).map(Self)
    }

    /// Returns the stored gust speed in meters per second.
    #[must_use]
    pub fn meters_per_second(&self) -> f64 {
        self.0
    }
}

/// Wind direction stored as degrees clockwise from north.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct WindDirection(f64);

impl WindDirection {
    /// Creates wind direction from a finite degree value in `0.0..360.0`.
    ///
    /// # Errors
    ///
    /// Returns [`WindValueError`] when the direction is invalid.
    pub fn new(degrees_from_north: f64) -> Result<Self, WindValueError> {
        if !degrees_from_north.is_finite() {
            return Err(WindValueError::NonFiniteDirection(degrees_from_north));
        }

        if !(0.0..360.0).contains(&degrees_from_north) {
            return Err(WindValueError::DirectionOutOfRange(degrees_from_north));
        }

        Ok(Self(degrees_from_north))
    }

    /// Returns the stored direction in degrees from north.
    #[must_use]
    pub fn degrees_from_north(&self) -> f64 {
        self.0
    }
}

/// Beaufort scale stored as an integer in `0..=12`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BeaufortScale(u8);

impl BeaufortScale {
    /// Creates a Beaufort scale value in `0..=12`.
    ///
    /// # Errors
    ///
    /// Returns [`WindValueError::BeaufortOutOfRange`] when the value is greater than `12`.
    pub fn new(value: u8) -> Result<Self, WindValueError> {
        if value > 12 {
            Err(WindValueError::BeaufortOutOfRange(value))
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the stored Beaufort scale value.
    #[must_use]
    pub fn value(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BeaufortScale, WindDirection, WindKind, WindKindParseError, WindSpeed, WindValueError,
    };
    use core::str::FromStr;

    #[test]
    fn valid_wind_speed() {
        let speed = WindSpeed::new(8.4).unwrap();

        assert_eq!(speed.meters_per_second(), 8.4);
    }

    #[test]
    fn negative_wind_speed_rejected() {
        assert_eq!(
            WindSpeed::new(-0.1),
            Err(WindValueError::NegativeSpeed(-0.1))
        );
    }

    #[test]
    fn valid_wind_direction() {
        let direction = WindDirection::new(135.0).unwrap();

        assert_eq!(direction.degrees_from_north(), 135.0);
    }

    #[test]
    fn invalid_direction_rejected() {
        assert_eq!(
            WindDirection::new(360.0),
            Err(WindValueError::DirectionOutOfRange(360.0))
        );
    }

    #[test]
    fn valid_beaufort_scale() {
        let value = BeaufortScale::new(5).unwrap();

        assert_eq!(value.value(), 5);
    }

    #[test]
    fn invalid_beaufort_scale_rejected() {
        assert_eq!(
            BeaufortScale::new(13),
            Err(WindValueError::BeaufortOutOfRange(13))
        );
    }

    #[test]
    fn wind_kind_display_and_parse() {
        assert_eq!(WindKind::Squall.to_string(), "squall");
        assert_eq!(WindKind::from_str("gale").unwrap(), WindKind::Gale);
        assert_eq!(WindKind::from_str(" "), Err(WindKindParseError::Empty));
    }
}
