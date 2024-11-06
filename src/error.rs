#[derive(thiserror::Error, Debug)]
pub enum ConfidenceError {
    #[error("a confidence interval must sit between 0 and 1, provided: {0:?}")]
    Value(Option<f64>),
}
