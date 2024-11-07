//! Abstractions for confidence intervals.
//!
//! ```
//! use confi::ConfidenceLevel;
//!
//! let from_fraction = ConfidenceLevel::fractional(0.1).unwrap();
//! let from_percentage = ConfidenceLevel::percentage(10.0).unwrap();
//!
//! assert_eq!(from_fraction, from_percentage);
//! ```

/// A confidence interval represents the range of values a measurand is expected to fall in,
/// calculated to a given [`ConfidenceLevel`]
mod interval;

use crate::{ConfidenceError, SignificanceLevel};
pub use interval::{Confidence, ConfidenceInterval};
use num_traits::{Float, FromPrimitive, ToPrimitive};
use std::fmt;

/// The degree of confidence associated with a value to be computed.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfidenceLevel<T>(pub(crate) T);

impl<T: ToPrimitive> fmt::Display for ConfidenceLevel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Confidence Level: {:.3}%",
            self.0.to_f64().unwrap() * 100.0
        )
    }
}

impl<T: Float + FromPrimitive + ToPrimitive> ConfidenceLevel<T> {
    /// Create a new confidence value from a fractional level
    ///
    /// # Errors
    /// - If the provided value is outside [0, 1], or is NaN
    pub fn fractional(level: T) -> Result<Self, ConfidenceError> {
        if (level < T::zero()) || (level > T::one()) {
            return Err(ConfidenceError::Value(level.to_f64()));
        }
        Ok(Self(level))
    }

    /// Create a new confidence value from a percentage level
    ///
    /// # Errors
    /// - If the provided value is outside [0, 100], or is NaN
    pub fn percentage(level: T) -> Result<Self, ConfidenceError> {
        let level = level / T::from_f64(100.0).unwrap();
        if (level < T::zero()) || (level > T::one()) {
            return Err(ConfidenceError::Value(level.to_f64()));
        }
        Ok(Self(level))
    }

    /// Create a new confidence value representing a confidence level of 99.9%
    pub fn ninety_nine_point_nine_percent() -> Self {
        Self(T::from_f64(0.999).unwrap())
    }

    /// Create a new confidence value representing a confidence level of 99.5%
    pub fn ninety_nine_point_five_percent() -> Self {
        Self(T::from_f64(0.995).unwrap())
    }

    /// Create a new confidence value representing a confidence level of 99.0%
    pub fn ninety_nine_percent() -> Self {
        Self(T::from_f64(0.99).unwrap())
    }

    /// Create a new confidence value representing a confidence level of 97.5%
    pub fn ninety_seven_point_five_percent() -> Self {
        Self(T::from_f64(0.975).unwrap())
    }

    /// Create a new confidence value representing a confidence level of 95.0%
    pub fn ninety_five_percent() -> Self {
        Self(T::from_f64(0.95).unwrap())
    }

    /// Create a new confidence value representing a confidence level of 90.0%
    pub fn ninety_percent() -> Self {
        Self(T::from_f64(0.9).unwrap())
    }

    /// Return the numerical value associated with the confidence level
    pub fn probability(&self) -> T {
        self.0
    }
}

impl<T: ToPrimitive> ConfidenceLevel<T> {
    pub fn to_f64(self) -> Option<ConfidenceLevel<f64>> {
        self.0.to_f64().map(|fraction| ConfidenceLevel(fraction))
    }
}

impl<T: Float> From<SignificanceLevel<T>> for ConfidenceLevel<T> {
    fn from(significance_level: SignificanceLevel<T>) -> Self {
        Self(T::one() - significance_level.0)
    }
}
