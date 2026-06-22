//! Confidence intervals for statistical estimation.
//!
//! A confidence interval represents a bounded range of values that is expected
//! to contain an unknown population parameter estimated from sample data.
//!
//! Each interval is associated with a [`ConfidenceLevel`], which specifies the
//! probability (in the long-run frequentist sense) that intervals constructed
//! in the same way will contain the true parameter.
//!
//! # Model interpretation
//!
//! A confidence interval is defined as:
//!
//! - a lower bound
//! - an upper bound
//! - an associated confidence level `c ∈ [0, 1]`
//!
//! # Structure
//!
//! Internally, a [`ConfidenceInterval`] is composed of:
//!
//! - a bounded interval `[lower, upper]`
//! - a [`ConfidenceLevel`]
//!
//! # Common operations
//!
//! The type supports basic interval arithmetic and inspection:
//!
//! - `width`: total span of the interval
//! - `center`: midpoint of the interval
//! - `contains`: membership test
//!
//! # Example
//!
//! ```
//! use confi::{ConfidenceInterval, ConfidenceLevel};
//!
//! let level = ConfidenceLevel::from_percent(95.0)?;
//! let interval = ConfidenceInterval::new(1.0, 3.0, level)?;
//!
//! assert!(interval.contains(2.0));
//! assert_eq!(interval.centre(), 2.0);
//! # Ok::<_, confi::ValidationError>(())
//! ```
use crate::{ConfidenceLevel, ValidationError};
use num_traits::{Float, ToPrimitive};
use std::fmt;

/// A closed numerical interval `[lower, upper]`.
///
/// This type represents a contiguous range of values in a total order,
///
/// # Invariants
///
/// - `lower <= upper`
/// - neither bound is NaN or infinite
///
/// These invariants are enforced at construction time.
///
/// # Semantics
///
/// The interval is **closed**, meaning both endpoints are included:
///
/// ```text
/// lower ≤ x ≤ upper
/// ```
///
/// # Note
///
/// This type is purely geometric and does not encode any statistical meaning
/// on its own.
#[derive(Clone, Debug)]
struct Interval<T> {
    /// Lower bound of the interval
    lower: T,
    /// Upper bound of the interval
    upper: T,
}

impl<T: Float> Interval<T> {
    // Create a new interval from lower and upper bounds
    //
    // # Errors
    // - If lower or upper is NaN
    // - If lower or upper is infinite
    // - If lower and upper are not an ordered interval
    fn new(lower: T, upper: T) -> Result<Self, ValidationError> {
        if lower.is_nan() || upper.is_nan() {
            return Err(ValidationError::NaN);
        }

        if !lower.is_finite() || !upper.is_finite() {
            return Err(ValidationError::Infinite);
        }

        if lower > upper {
            return Err(ValidationError::UnorderedRange {
                lower: lower.to_f64().unwrap(),
                upper: upper.to_f64().unwrap(),
            });
        }

        Ok(Self { lower, upper })
    }
}

impl<T> fmt::Display for Interval<T>
where
    T: Float + ToPrimitive,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lower = self.lower.to_f64().ok_or(fmt::Error)?;
        let upper = self.upper.to_f64().ok_or(fmt::Error)?;

        match f.precision() {
            Some(p) => write!(f, "[{lower:.p$}, {upper:.p$}]"),
            None => write!(f, "[{lower:.3}, {upper:.3}]"),
        }
    }
}

/// A statistical confidence interval.
///
/// A confidence interval represents a bounded range of values associated with
/// a statistical estimate, together with a confidence level describing the
/// reliability of the construction procedure.
///
/// # Structure
///
/// A confidence interval consists of:
///
/// - a lower bound
/// - an upper bound
/// - an associated [`ConfidenceLevel`]
///
/// # Geometry
///
/// The interval is closed:
///
/// ```text
/// lower ≤ x ≤ upper
/// ```
#[derive(Clone, Debug)]
pub struct ConfidenceInterval<T> {
    /// The range of values expected to enclose the estimated parameter
    interval: Interval<T>,
    /// The level of confidence.
    ///
    /// The range is expected to enclose the estimated parameter with a probability given by the
    /// confidence level
    confidence_level: ConfidenceLevel<T>,
}

impl<T> fmt::Display for ConfidenceInterval<T>
where
    T: Float + ToPrimitive,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:.2}%)", self.interval, self.confidence_level)
    }
}

impl<T: Copy> ConfidenceInterval<T> {
    /// Returns the confidence level associated with the confidence interval
    pub fn level(&self) -> ConfidenceLevel<T> {
        self.confidence_level
    }
}

impl<T: Float> ConfidenceInterval<T> {
    /// Creates a confidence interval from a lower and upper bounds and a confidence level.
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::{ConfidenceInterval, ConfidenceLevel};
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let (lower, upper) = (0.0, 1.0);
    ///
    /// let ci = ConfidenceInterval::new(lower, upper, cl)?;
    ///
    /// assert_eq!(ci.lower(), lower);
    /// assert_eq!(ci.upper(), upper);
    /// assert_eq!(ci.level().into_inner(), 0.95);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    ///
    /// # Errors
    /// - If the lower or upper bound are NaN
    /// - If the lower or upper bound are infinite
    /// - If the lower or upper bound do not form an ordered interval
    pub fn new(
        lower: T,
        upper: T,
        confidence_level: ConfidenceLevel<T>,
    ) -> Result<Self, ValidationError> {
        let interval = Interval::new(lower, upper)?;
        Ok(Self {
            interval,
            confidence_level,
        })
    }
}

impl<T: Float> ConfidenceInterval<T> {
    /// Returns the lower bound for the confidence interval
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::{ConfidenceInterval, ConfidenceLevel};
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let (lower, upper) = (0.0, 1.0);
    ///
    /// let ci = ConfidenceInterval::new(lower, upper, cl)?;
    ///
    /// assert_eq!(ci.lower(), lower);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn lower(&self) -> T {
        self.interval.lower
    }

    /// Returns the upper bound for the confidence interval
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::{ConfidenceInterval, ConfidenceLevel};
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let (lower, upper) = (0.0, 1.0);
    ///
    /// let ci = ConfidenceInterval::new(lower, upper, cl)?;
    ///
    /// assert_eq!(ci.upper(), upper);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn upper(&self) -> T {
        self.interval.upper
    }
}

impl<T: Float> ConfidenceInterval<T> {
    /// Returns true if the interval contains the argument
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::{ConfidenceInterval, ConfidenceLevel};
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let (lower, upper) = (0.0, 1.0);
    ///
    /// let ci = ConfidenceInterval::new(lower, upper, cl)?;
    ///
    /// assert_eq!(ci.contains(0.5), true);
    /// assert_eq!(ci.contains(1.5), false);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn contains(&self, value: T) -> bool {
        if value.is_nan() {
            return false;
        }

        (value <= self.upper()) && (value >= self.lower())
    }

    /// Returns the width of the confidence interval
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::{ConfidenceInterval, ConfidenceLevel};
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let (lower, upper) = (0.0, 1.0);
    ///
    /// let ci = ConfidenceInterval::new(lower, upper, cl)?;
    ///
    /// assert_eq!(ci.width(), upper - lower);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn width(&self) -> T {
        self.upper() - self.lower()
    }

    /// Returns the centre of the confidence interval
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::{ConfidenceInterval, ConfidenceLevel};
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let (lower, upper) = (0.0, 1.0);
    ///
    /// let ci = ConfidenceInterval::new(lower, upper, cl)?;
    ///
    /// assert_eq!(ci.centre(), 0.5);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn centre(&self) -> T {
        let two = T::one() + T::one();
        (self.lower() + self.upper()) / two
    }

    /// Returns the radius of the confidence interval
    ///
    /// # Examples
    ///
    /// ```
    /// use confi::{ConfidenceInterval, ConfidenceLevel};
    ///
    /// let cl = ConfidenceLevel::new(0.95)?;
    /// let (lower, upper) = (0.0, 1.0);
    ///
    /// let ci = ConfidenceInterval::new(lower, upper, cl)?;
    ///
    /// assert_eq!(ci.radius(), 0.5);
    /// # Ok::<_, confi::ValidationError>(())
    /// ```
    pub fn radius(&self) -> T {
        let two = T::one() + T::one();
        self.width() / two
    }
}

impl<T> ConfidenceInterval<T>
where
    T: ToPrimitive,
{
    pub fn to_f64(self) -> Option<ConfidenceInterval<f64>> {
        Some(ConfidenceInterval {
            interval: Interval {
                lower: self.interval.lower.to_f64()?,
                upper: self.interval.upper.to_f64()?,
            },
            confidence_level: self.confidence_level.to_f64()?,
        })
    }

    pub fn to_f32(self) -> Option<ConfidenceInterval<f32>> {
        Some(ConfidenceInterval {
            interval: Interval {
                lower: self.interval.lower.to_f32()?,
                upper: self.interval.upper.to_f32()?,
            },
            confidence_level: self.confidence_level.to_f32()?,
        })
    }
}
