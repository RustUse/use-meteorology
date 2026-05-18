#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_air_temperature as air_temperature;
pub use use_atmosphere as atmosphere;
pub use use_atmospheric_pressure as atmospheric_pressure;
pub use use_cloud as cloud;
pub use use_humidity as humidity;
pub use use_precipitation as precipitation;
pub use use_pressure_system as pressure_system;
pub use use_weather_forecast as weather_forecast;
pub use use_weather_front as weather_front;
pub use use_weather_observation as weather_observation;
pub use use_wind as wind;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{
        AirTemperature, AtmosphericPressure, CloudCover, CloudKind, ForecastConfidence,
        ForecastHorizon, ForecastId, ForecastKind, ObservationKind, ObservationQuality,
        ObservationSource, PrecipitationAmount, PrecipitationIntensity, PrecipitationKind,
        PressureSystemKind, PressureSystemName, RelativeHumidity, WeatherFrontKind,
        WeatherObservation, WeatherObservationId, WindDirection, WindSpeed,
    };

    #[test]
    fn facade_composes_meteorology_primitives_without_prediction() {
        let observation = WeatherObservation::new(
            WeatherObservationId::new("obs-100").unwrap(),
            ObservationKind::Surface,
            ObservationSource::new("automated station").unwrap(),
            ObservationQuality::Verified,
        );
        let temperature = AirTemperature::new(17.5).unwrap();
        let pressure = AtmosphericPressure::new(1011.6).unwrap();
        let humidity = RelativeHumidity::new(71.0).unwrap();
        let wind_speed = WindSpeed::new(4.8).unwrap();
        let wind_direction = WindDirection::new(140.0).unwrap();
        let cloud_kind = CloudKind::Altocumulus;
        let cloud_cover = CloudCover::new(5).unwrap();
        let precipitation_kind = PrecipitationKind::Rain;
        let precipitation_amount = PrecipitationAmount::new(2.4).unwrap();
        let precipitation_intensity = PrecipitationIntensity::Light;
        let front_kind = WeatherFrontKind::Cold;
        let pressure_system_name = PressureSystemName::new("Prairie Low").unwrap();
        let pressure_system_kind = PressureSystemKind::Low;
        let forecast_id = ForecastId::new("fcst-100").unwrap();
        let forecast_kind = ForecastKind::ShortRange;
        let forecast_horizon = ForecastHorizon::new(12);
        let forecast_confidence = ForecastConfidence::Medium;

        assert_eq!(observation.kind(), &ObservationKind::Surface);
        assert_eq!(temperature.celsius(), 17.5);
        assert_eq!(pressure.hectopascals(), 1011.6);
        assert_eq!(humidity.percent(), 71.0);
        assert_eq!(wind_speed.meters_per_second(), 4.8);
        assert_eq!(wind_direction.degrees_from_north(), 140.0);
        assert_eq!(cloud_kind.to_string(), "altocumulus");
        assert_eq!(cloud_cover.oktas(), 5);
        assert_eq!(precipitation_kind.to_string(), "rain");
        assert_eq!(precipitation_amount.millimeters(), 2.4);
        assert_eq!(precipitation_intensity.to_string(), "light");
        assert_eq!(front_kind.to_string(), "cold");
        assert_eq!(pressure_system_name.as_str(), "Prairie Low");
        assert_eq!(pressure_system_kind.to_string(), "low");
        assert_eq!(forecast_id.as_str(), "fcst-100");
        assert_eq!(forecast_kind.to_string(), "short-range");
        assert_eq!(forecast_horizon.hours(), 12);
        assert_eq!(forecast_confidence.to_string(), "medium");
    }
}
