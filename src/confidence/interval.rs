//! A [`ConfidenceInterval`] describes the range of a values a quantity can take, expressed to a
//! given [`ConfidenceLevel`]
//! ```
//! use confidence::{ConfidenceLevel, ConfidenceInterval, Confidence};
//!
//! let from_fraction = ConfidenceLevel::fractional(0.1).unwrap();
//! let interval = ConfidenceInterval::new(1.0..=3.0, from_fraction);
//!
//! assert!(interval.contains(2.0));
//!
//! ```

use crate::ConfidenceLevel;
use num_traits::{Float, FromPrimitive, ToPrimitive};
use std::{fmt, ops::RangeInclusive};

/// A [`ConfidenceInterval`] describes a range of possible values expected to enclose the estimated
/// parameter. The [`ConfidenceInterval`] is defined by an associated [`ConfidenceLevel`] and an
/// inclusive range.
#[derive(Clone, Debug)]
pub struct ConfidenceInterval<T> {
    /// The range of values expected to enclose the estimated parameter
    pub(crate) range: RangeInclusive<T>,
    /// The level of confidence.
    ///
    /// The range is expected to enclose the estimated parameter with a probability given by the
    /// confidence level
    confidence_level: ConfidenceLevel<T>,
}

impl<T: ToPrimitive> fmt::Display for ConfidenceInterval<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Confidence Interval: {:.3e} -> {:.3e} ({})",
            self.range.start().to_f64().unwrap(),
            self.range.end().to_f64().unwrap(),
            self.confidence_level
        )
    }
}

pub trait Confidence<T: Float + FromPrimitive> {
    fn new(range: RangeInclusive<T>, confidence_level: ConfidenceLevel<T>) -> Self;
    fn width(&self) -> T {
        *self.end() - *self.start()
    }
    fn half_width(&self) -> T {
        self.width() / T::from(2).unwrap()
    }
    fn start(&self) -> &T;
    fn end(&self) -> &T;
    fn confidence_level(&self) -> ConfidenceLevel<T>;
    fn contains(&self, val: T) -> bool {
        (val <= *self.end()) && (val >= *self.start())
    }
}

impl<T: Float + FromPrimitive> Confidence<T> for ConfidenceInterval<T> {
    fn new(range: RangeInclusive<T>, confidence_level: ConfidenceLevel<T>) -> Self {
        Self {
            range,
            confidence_level,
        }
    }

    fn start(&self) -> &T {
        self.range.start()
    }

    fn end(&self) -> &T {
        self.range.end()
    }

    fn confidence_level(&self) -> ConfidenceLevel<T> {
        self.confidence_level
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interval_display() {
        let level = ConfidenceLevel::ninety_nine_percent();
        let range = 1e-5..=4e-2;

        let interval = ConfidenceInterval::new(range, level);

        println!("{interval}");
    }
}
