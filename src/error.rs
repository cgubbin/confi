use num_traits::Float;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    #[error("value must lie in the range [0, 1], got: {value}")]
    OutOfRange { value: f64 },

    #[error("value cannot be NaN")]
    NaN,

    #[error("value cannot be infinite")]
    Infinite,

    #[error("expected an ordered range, received [{lower:.3}, {upper:.3}]")]
    UnorderedRange { lower: f64, upper: f64 },
}

// Validate a probability value. Returns the value if it lies in the closed interval [0, 1]
//
// # Errors
// - If the argument is outside the interval [0, 1]
// - If the argument is NaN
pub(crate) fn validate_probability<T: Float>(level: T) -> Result<T, ValidationError> {
    if level.is_nan() {
        return Err(ValidationError::NaN);
    }

    let value = level.to_f64().unwrap();

    if !(0.0..=1.0).contains(&value) {
        return Err(ValidationError::OutOfRange { value });
    }

    Ok(level)
}
