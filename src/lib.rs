//! # Confi
//!
//! `confi` provides lightweight, type-safe primitives for expressing
//! statistical confidence levels, significance levels, and confidence intervals.
//!
//! The crate is designed to make statistical intent explicit in code, without
//! pulling in heavy statistical frameworks.
//!
//! It focuses on **representation and validation**, not on distribution modeling.
//!
//! ---
//!
//! ## Core types
//!
//! - [`ConfidenceLevel`] — probability that a procedure correctly captures the true value
//! - [`SignificanceLevel`] — probability threshold for hypothesis testing (α)
//! - [`ConfidenceInterval`] — bounded estimate with an associated confidence level
//!
//! All types enforce that probabilities remain in the valid range `[0, 1]`.
//!
//! ---
//!
//! ## Key concepts
//!
//! ### Confidence vs significance
//!
//! Confidence and significance are duals:
//!
//! ```text
//! confidence = 1 - significance
//! ```
//!
//! ```rust
//! use confi::{ConfidenceLevel, SignificanceLevel};
//!
//! let c = ConfidenceLevel::from_percent(95.0)?;
//! let a: SignificanceLevel<f64> = c.significance();
//!
//! approx::assert_relative_eq!(a.into_inner(), 0.05);
//! # Ok::<_, confi::ValidationError>(())
//! ```
//!
//! ---
//!
//! ## Confidence intervals
//!
//! A confidence interval combines:
//!
//! - a lower bound
//! - an upper bound
//! - a confidence level
//!
//! ```rust
//! use confi::{ConfidenceInterval, ConfidenceLevel};
//!
//! let level = ConfidenceLevel::from_percent(95.0)?;
//! let ci = ConfidenceInterval::new(1.0, 3.0, level)?;
//!
//! assert!(ci.contains(2.0));
//! # Ok::<_, confi::ValidationError>(())
//! ```
//!
//! Intervals are closed: `lower ≤ x ≤ upper`.
//!
//! ---
//!
//! ## Features
//!
//! ### Safe probability types
//! All probability-like values are validated to ensure:
//!
//! - `0 ≤ p ≤ 1`
//! - no NaN values
//! - finite values only
//!
//! ---
//!
//! ## Design philosophy
//!
//! This crate is intentionally minimal:
//!
//! - No statistical distributions are included
//! - No hypothesis testing logic is implemented
//! - No external statistical dependencies are required
//!
//! It is intended as a **building block library** for statistical systems,
//! not a full statistics toolkit.
//!
//! ---
//!
//! ## Example
//!
//! ```rust
//! use confi::{ConfidenceLevel, ConfidenceInterval, SignificanceLevel};
//!
//! let confidence = ConfidenceLevel::from_percent(95.0)?;
//! let alpha: SignificanceLevel<f64> = confidence.into();
//!
//! let ci = ConfidenceInterval::new(0.0, 1.0, confidence)?;
//!
//! assert!(ci.contains(0.5));
//! approx::assert_relative_eq!(alpha.into_inner(), 0.05);
//! # Ok::<_, confi::ValidationError>(())
//! ```
//!
mod confidence_interval;
mod confidence_level;
mod error;
mod significance;

pub use confidence_interval::ConfidenceInterval;
pub use confidence_level::ConfidenceLevel;
pub use error::ValidationError;
pub use significance::SignificanceLevel;

pub(crate) use error::validate_probability;
