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
//! let from_probability = SignificanceLevel::new(0.1).unwrap();
//! let from_percent= SignificanceLevel::from_percent(10.0).unwrap();
//!
//! assert_eq!(from_probability, from_percent);
//!
//! ```

use crate::{ConfidenceLevel, ValidationError, validate_probability};
use num_traits::{Float, FromPrimitive, ToPrimitive};
use std::fmt;

/// A statistical significance level.
///
/// Represents a probability in the closed interval `[0, 1]`.
///
/// Common values include:
///
/// - 0.10 (10%)
/// - 0.05 (5%)
/// - 0.01 (1%)
///
/// A significance level `α` corresponds to a confidence level
/// `α = 1 - c`.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct SignificanceLevel<T>(T);

impl<T: ToPrimitive> fmt::Display for SignificanceLevel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let percent = self.0.to_f64().ok_or(fmt::Error)? * 100.0;

        match f.precision() {
            Some(p) => write!(f, "{percent:.p$}%"),
            None => write!(f, "{percent:.1}%"),
        }
    }
}

macro_rules! impl_significance_constants {
    ($t:ty) => {
        impl SignificanceLevel<$t> {
            pub const SL10: Self = Self(0.10);
            pub const SL05: Self = Self(0.05);
            pub const SL025: Self = Self(0.025);
            pub const SL01: Self = Self(0.01);
            pub const SL001: Self = Self(0.001);
        }
    };
}

impl_significance_constants!(f32);
impl_significance_constants!(f64);

impl<T> SignificanceLevel<T> {
    /// Returns the probability associated with the significance level as a fraction
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Float> SignificanceLevel<T> {
    /// Creates a significance level from a probability.
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::SignificanceLevel;
    ///
    /// let sl = SignificanceLevel::new(0.05)?;
    /// assert_eq!(sl.into_inner(), 0.05);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    ///
    /// # Errors
    /// - If the provided probability is outside [0, 1], or is NaN
    pub fn new(probability: T) -> Result<Self, ValidationError> {
        let probability = validate_probability(probability)?;
        Ok(Self(probability))
    }
}

impl<T: Float + FromPrimitive + ToPrimitive> SignificanceLevel<T> {
    /// Creates a significance level from a percentage.
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::SignificanceLevel;
    ///
    /// let sl = SignificanceLevel::from_percent(5.0)?;
    /// assert_eq!(sl.into_inner(), 0.05);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    ///
    /// # Errors
    /// - If the provided probability is outside [0, 100], or is NaN
    pub fn from_percent(percentage: T) -> Result<Self, ValidationError> {
        let level = validate_probability(percentage / T::from_f64(100.0).unwrap())?;
        Ok(Self(level))
    }

    /// Convert to the associated confidence level
    ///
    /// As significance levels are checked for validity on creation the conversion is infallible
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::SignificanceLevel;
    ///
    /// let sl = SignificanceLevel::new(0.05)?;
    /// let cl = sl.confidence();
    /// approx::assert_relative_eq!(cl.into_inner(), 0.95);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn confidence(self) -> ConfidenceLevel<T> {
        ConfidenceLevel::new(T::one() - self.0).unwrap() // As Confidence levels are validated
        // self.0 is always in [0, 1]
    }
}

impl<T: ToPrimitive> SignificanceLevel<T> {
    pub fn to_f64(self) -> Option<SignificanceLevel<f64>> {
        self.0.to_f64().map(SignificanceLevel)
    }

    pub fn to_f32(self) -> Option<SignificanceLevel<f32>> {
        self.0.to_f32().map(SignificanceLevel)
    }
}

impl<T: Float> From<SignificanceLevel<T>> for ConfidenceLevel<T> {
    fn from(significance_level: SignificanceLevel<T>) -> Self {
        ConfidenceLevel::new(T::one() - significance_level.0).unwrap() // Significance levels
        // cannot be created with probabilities outside [0, 1]
    }
}
