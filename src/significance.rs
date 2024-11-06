//! The significance level describes the evidence present in a sample under test which allows for
//! rejection of the null hypothesis (the state when two outcomes or possibilities are the same).
//!
//! In a measurement model the null hypothesis under test is typically whether a given reading is
//! indistinguishable from another, or whether it is indistinguishable from the system response at
//! zero stimulus (the minimum detectable value).
//!
//! ```rust
//! use confi::SignificanceLevel;
//!
//! let from_fraction = SignificanceLevel::fractional(0.1).unwrap();
//! let from_percentage = SignificanceLevel::percentage(10.0).unwrap();
//!
//! assert_eq!(from_fraction, from_percentage);
//!
//! ```

use crate::{ConfidenceError, ConfidenceLevel};
use num_traits::{Float, FromPrimitive, ToPrimitive};
use statrs::{
    distribution::{ContinuousCDF, Normal},
    StatsError,
};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
/// The significance level is expressed as a fraction.
///
/// It represents the probability of a type I error, which corresponds to rejection of a null
/// hypothesis which is in fact true.
pub struct SignificanceLevel<T>(pub(crate) T);

impl<T: ToPrimitive> fmt::Display for SignificanceLevel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Significance Level: {:.3}%",
            self.0.to_f64().unwrap() * 100.0
        )
    }
}

impl<T: Float + FromPrimitive + ToPrimitive> SignificanceLevel<T> {
    /// Create a new significance value from a fractional level
    ///
    /// # Errors
    /// - If the provided value is outside [0, 1], or is NaN
    pub fn fractional(level: T) -> Result<Self, ConfidenceError> {
        if (level < T::zero()) || (level > T::one()) {
            return Err(ConfidenceError::Value(level.to_f64()));
        }
        Ok(Self(level))
    }

    /// Create a new significance value from a percentage level
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

    /// Create a new significance value representing a significance level of 0.1%
    pub fn zero_point_one_percent() -> Self {
        Self(T::from_f64(0.001).unwrap())
    }

    /// Create a new significance value representing a significance level of 0.5%
    pub fn zero_point_five_percent() -> Self {
        Self(T::from_f64(0.005).unwrap())
    }

    /// Create a new significance value representing a significance level of 1.0%
    pub fn one_percent() -> Self {
        Self(T::from_f64(0.01).unwrap())
    }

    /// Create a new significance value representing a significance level of 2.5%
    pub fn two_point_five_percent() -> Self {
        Self(T::from_f64(0.025).unwrap())
    }

    /// Create a new significance value representing a significance level of 5.0%
    pub fn five_percent() -> Self {
        Self(T::from_f64(0.05).unwrap())
    }

    /// Create a new significance value representing a significance level of 10.0%
    pub fn ten_percent() -> Self {
        Self(T::from_f64(0.1).unwrap())
    }

    /// Return the numerical value associated with the significance level
    pub fn probability(&self) -> T {
        self.0
    }

    /// The number of standard deviations from the distribution center represented by the
    /// significance level.
    ///
    /// This is found by calculating the inverse cumululative distribution function at the significance
    /// level. The inverse CDF gives the value of the measurand which leads to the given
    /// probability. The number of standard deviations is this divided by the standard deviation of
    /// the distribution
    pub fn num_standard_deviations(&self) -> Result<T, StatsError> {
        let distribution = Normal::new(0.0, 1.0)?;
        Ok(T::from_f64(distribution.inverse_cdf(1.0 - self.0.to_f64().unwrap())).unwrap())
    }
}

impl<T: Float> From<ConfidenceLevel<T>> for SignificanceLevel<T> {
    fn from(confidence_level: ConfidenceLevel<T>) -> Self {
        Self(T::one() - confidence_level.0)
    }
}
