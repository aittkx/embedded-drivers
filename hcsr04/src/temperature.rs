//! Temperature compensation for HC-SR04 sensor
//!
//! The [`TemperatureProvider`] trait allows for temperature compensation in the HC-SR04 sensor driver.
//! The [`NoTemperatureCompensation`] struct provides a no-op implementation for applications that don't need temperature compensation.

/// Trait for temperature compensation
///
/// Sound speed varies with temperature, affecting distance calculations.
/// This trait allows for temperature-compensated measurements.
pub trait TemperatureProvider {
    /// Error type for temperature operations
    type Error;

    /// Get current temperature in Celsius
    ///
    /// # Returns
    ///
    /// Temperature in degrees Celsius
    fn temperature_celsius(&self) -> core::result::Result<f32, Self::Error>;

    /// Calculate temperature-compensated sound speed
    ///
    /// # Arguments
    ///
    /// * `temperature_c` - Temperature in Celsius
    ///
    /// # Returns
    ///
    /// Sound speed in cm/μs
    fn sound_speed_cm_per_us(&self, temperature_c: f32) -> f32 {
        // Standard formula: v = 331.3 * sqrt(1 + T/273.15) m/s
        // Simplified approximation: v ≈ 331.3 * (1 + 0.00183 * T) m/s
        let speed_m_per_s = 331.3 * (1.0 + 0.00183 * temperature_c);
        speed_m_per_s / 10000.0 // Convert to cm/μs
    }
}

/// No-op temperature provider for applications that don't need temperature compensation
pub struct NoTemperatureCompensation;

impl TemperatureProvider for NoTemperatureCompensation {
    type Error = ();
    fn temperature_celsius(&self) -> core::result::Result<f32, Self::Error> {
        // Return standard temperature (20°C)
        Ok(20.0)
    }
}
