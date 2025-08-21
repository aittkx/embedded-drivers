//! Error types for HC-SR04 driver

/// Errors that can occur when using the HC-SR04 driver
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The HC-SR04 did not respond within the expected time.
    Timeout,
    /// An error occurred when using the GPIO pin.
    GPIO(&'static str),
    /// An error occurred when using the hardware timer.
    Timestamp(&'static str),
    /// An error occurred when using the temperature provider.
    Temperature(&'static str),
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Timeout => write!(f, "Timeout"),
            Self::GPIO(e) => write!(f, "GPIO with {}", e),
            Self::Timestamp(e) => write!(f, "Timestamp with {}", e),
            Self::Temperature(e) => write!(f, "Temperature with {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_device() {
        assert_eq!(format!("{}", Error::Timeout), "Timeout");
    }

    #[test]
    fn test_error_debug() {
        // Test that Debug trait is implemented and works
        let error = Error::Timeout;
        let debug_output = format!("{error:?}",);
        assert!(debug_output.contains("Timeout"));
    }

    #[test]
    fn test_error_partialeq() {
        // Test that all variants implement PartialEq correctly
        assert!(Error::Timeout.eq(&Error::Timeout));
        assert!(!Error::Timeout.eq(&Error::GPIO("test")));
    }
}
