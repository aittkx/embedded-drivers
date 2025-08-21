//! HC-SR04 ultrasonic distance sensor driver core implementation

use crate::{NoTemperatureCompensation, Result, TemperatureProvider};
use bon::Builder;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::digital::Wait;

const TIMEOUT_US: u32 = 35000; // 35ms timeout

/// The HC-SR04 ultrasonic distance sensor driver
///
/// Support temperature compensation
#[derive(Builder)]
pub struct Hcsr04<TRIGPIN, ECHOPIN, DELAY, TEMP = NoTemperatureCompensation> {
    trig: TRIGPIN,
    echo: ECHOPIN,
    delay: DELAY,
    temperature: TEMP,
}

impl<TRIGPIN, ECHOPIN, DELAY, TEMP> Hcsr04<TRIGPIN, ECHOPIN, DELAY, TEMP>
where
    TRIGPIN: OutputPin,
    ECHOPIN: InputPin + Wait,
    DELAY: DelayNs,
    TEMP: TemperatureProvider,
{
    /// Measure distance and return value directly in centimeters
    ///
    /// This is a convenience method that internally calls `measure()` and converts to distance
    pub async fn measure_distance(&mut self) -> Result<f32> {
        let pulse_width = self.measure().await?;
        self.pulse_width_to_cm(pulse_width)
    }

    /// Measure distance and return pulse width in microseconds
    pub async fn measure(&mut self) -> Result<u32> {
        // Send trigger pulse
        self.send_trigger_pulse().await?;

        // Wait for echo to go high
        self.wait_for_echo_start().await?;

        // Measure pulse width
        let pulse_width = self.wait_for_echo_end().await?;

        Ok(pulse_width)
    }

    /// Send 10μs trigger pulse
    async fn send_trigger_pulse(&mut self) -> Result<()> {
        // Ensure trigger pin is low before starting
        self.trig
            .set_low()
            .map_err(|_| crate::Error::GPIO("Failed to set trigger low"))?;
        self.delay.delay_us(2).await;

        self.trig
            .set_high()
            .map_err(|_| crate::Error::GPIO("Failed to set trigger high"))?;
        self.delay.delay_us(10).await;

        self.trig
            .set_low()
            .map_err(|_| crate::Error::GPIO("Failed to set trigger low"))?;

        Ok(())
    }

    /// Wait for echo pin to go high with timeout
    async fn wait_for_echo_start(&mut self) -> Result<u32> {
        let mut elapsed = 0u32;
        while self
            .echo
            .is_low()
            .map_err(|_| crate::Error::GPIO("Failed to read echo pin"))?
        {
            if elapsed >= TIMEOUT_US {
                return Err(crate::Error::Timeout);
            }
            // Short delay to prevent excessive CPU usage
            self.delay.delay_us(1).await;
            elapsed += 1;
        }
        Ok(elapsed)
    }

    /// Wait for echo pin to go low with timeout
    async fn wait_for_echo_end(&mut self) -> Result<u32> {
        let mut elapsed = 0u32;
        while self
            .echo
            .is_high()
            .map_err(|_| crate::Error::GPIO("Failed to read echo pin"))?
        {
            if elapsed >= TIMEOUT_US {
                return Err(crate::Error::Timeout);
            }
            // Short delay to prevent excessive CPU usage
            self.delay.delay_us(1).await;
            elapsed += 1;
        }
        Ok(elapsed)
    }

    /// Convert pulse width to distance in centimeters
    ///
    /// Default [`NoTemperatureCompensation`] uses standard temperature (20°C)
    ///
    /// # Arguments
    ///
    /// * `pulse_width_us` - Pulse width in microseconds
    ///
    /// # Returns
    ///
    /// Distance value in centimeters
    pub fn pulse_width_to_cm(&self, pulse_width_us: u32) -> Result<f32> {
        let temperature = self
            .temperature
            .temperature_celsius()
            .map_err(|_| crate::Error::Temperature("Failed to read temperature"))?;

        let sound_speed_cm_per_us = self.temperature.sound_speed_cm_per_us(temperature);
        let distance_cm = (pulse_width_us as f32 * sound_speed_cm_per_us) / 2.0;
        Ok(distance_cm)
    }
}
