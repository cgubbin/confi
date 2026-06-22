//! Statistical confidence levels.
//!
//! A confidence level expresses the probability that a confidence interval
//! or statistical estimate contains the true value of a parameter under the
//! assumptions of the underlying model.
//!
//! Confidence levels are represented as probabilities in the closed interval
//! `[0, 1]`.
//!
//! Common values include:
//!
//! - 90% (`0.90`)
//! - 95% (`0.95`)
//! - 99% (`0.99`)
//!
//! Confidence levels can be created from either probabilities or percentages:
//!
//! ```
//! use confi::ConfidenceLevel;
//!
//! let from_probability = ConfidenceLevel::new(0.95)?;
//! let from_percent = ConfidenceLevel::from_percent(95.0)?;
//!
//! assert_eq!(from_probability, from_percent);
//! # Ok::<_, confi::ValidationError>(())
//! ```
//!
//! Every confidence level corresponds to a significance level
//! `α = 1 - c`:
//!
//! ```
//! use confi::{ConfidenceLevel, SignificanceLevel};
//!
//! let confidence = ConfidenceLevel::new(0.95)?;
//! let significance: SignificanceLevel<f64> = confidence.into();
//!
//! approx::assert_relative_eq!(significance.into_inner(), 0.05);
//! # Ok::<_, confi::ValidationError>(())
//! ```

use crate::{SignificanceLevel, ValidationError, validate_probability};
use num_traits::{Float, FromPrimitive, ToPrimitive};
use std::fmt;

/// A statistical confidence level.
///
/// Represents a probability in the closed interval `[0, 1]`.
///
/// Common values include:
///
/// - 0.90 (90%)
/// - 0.95 (95%)
/// - 0.99 (99%)
///
/// A confidence level `c` corresponds to a significance level
/// `α = 1 - c`.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ConfidenceLevel<T>(T);

// impl<T: Float + ToPrimitive> TryFrom<T> for ConfidenceLevel<T> {
//     type Error = ValidationError;

//     fn try_from(value: T) -> Result<Self, Self::Error> {
//         Self::new(value)
//     }
// }

impl<T: ToPrimitive> fmt::Display for ConfidenceLevel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let percent = self.0.to_f64().ok_or(fmt::Error)? * 100.0;

        match f.precision() {
            Some(p) => write!(f, "{percent:.p$}%"),
            None => write!(f, "{percent:.1}%"),
        }
    }
}

macro_rules! impl_confidence_constants {
    ($t:ty) => {
        impl ConfidenceLevel<$t> {
            pub const CL90: Self = Self(0.90);
            pub const CL95: Self = Self(0.95);
            pub const CL975: Self = Self(0.975);
            pub const CL99: Self = Self(0.99);
            pub const CL999: Self = Self(0.999);
        }
    };
}

impl_confidence_constants!(f32);
impl_confidence_constants!(f64);

impl<T> ConfidenceLevel<T> {
    /// Returns the probability associated with the confidence level
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Float> ConfidenceLevel<T> {
    /// Creates a confidence level from a probability.
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::ConfidenceLevel;
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// assert_eq!(cl.into_inner(), 0.95);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    ///
    /// # Errors
    /// - If the provided probability is outside [0, 1]
    /// - If the provided probability is NaN
    pub fn new(probability: T) -> Result<Self, ValidationError> {
        let level = validate_probability(probability)?;
        Ok(Self(level))
    }

    /// Convert to the associated significance level
    ///
    /// As confidence levels are checked for validity on creation the conversion is infallible
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::ConfidenceLevel;
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let significance = cl.significance();
    /// approx::assert_relative_eq!(significance.into_inner(), 0.05);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn significance(self) -> SignificanceLevel<T> {
        SignificanceLevel::new(T::one() - self.0).unwrap() // As Confidence levels are validated
        // self.0 is always in [0, 1]
    }
}

impl<T: Float + FromPrimitive> ConfidenceLevel<T> {
    /// Creates a confidence level from a percentage.
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::ConfidenceLevel;
    ///
    /// let cl = ConfidenceLevel::from_percent(95.0)?;
    /// assert_eq!(cl.into_inner(), 0.95);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    ///
    /// # Errors
    /// - If the provided percentage is outside [0, 100]
    /// - If the provided percentage is NaN
    pub fn from_percent(percentage: T) -> Result<Self, ValidationError> {
        let level = validate_probability(percentage / T::from_f64(100.0).unwrap())?;

        Ok(Self(level))
    }
}

impl<T: ToPrimitive> ConfidenceLevel<T> {
    pub fn to_f64(self) -> Option<ConfidenceLevel<f64>> {
        self.0.to_f64().map(ConfidenceLevel)
    }

    pub fn to_f32(self) -> Option<ConfidenceLevel<f32>> {
        self.0.to_f32().map(ConfidenceLevel)
    }
}

impl<T: Float> From<ConfidenceLevel<T>> for SignificanceLevel<T> {
    fn from(confidence_level: ConfidenceLevel<T>) -> Self {
        SignificanceLevel::new(T::one() - confidence_level.0).unwrap() // Significance levels are
        // validated to have probabilities in the valid [0, 1] range
    }
}
