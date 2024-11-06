mod confidence;
mod error;
mod significance;

pub use confidence::{Confidence, ConfidenceInterval, ConfidenceLevel};
pub use error::ConfidenceError;
pub use significance::SignificanceLevel;
