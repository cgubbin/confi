//! Implementation of  the Bartlett test
//!
//! The Bartlett test validates the hypothesis that two sample sets are drawn from distributions
//! with equal variance

use ndarray::{Array1, ArrayView1};
use num_traits::{Float, FromPrimitive, ToPrimitive};
use statrs::distribution::ContinuousCDF;

#[derive(Clone, Debug)]
pub(crate) struct BartlettTestResult<N> {
    statistic: N,
    pub(crate) pvalue: N,
}

#[derive(Debug, thiserror::Error)]
pub enum BartlettTestError {
    #[error("at least two inputs are needed for a Bartlett test; got {0}")]
    Input(usize),
    #[error("chi2 computation {0}")]
    Stats(#[from] statrs::StatsError),
}

pub(crate) fn bartlett_test<N>(
    inputs: &[ArrayView1<'_, N>],
) -> Result<BartlettTestResult<N>, BartlettTestError>
where
    N: Float + std::iter::Sum<N> + FromPrimitive + ToPrimitive,
{
    if inputs.len() < 2 {
        return Err(BartlettTestError::Input(inputs.len()));
    }

    // Assigning here to reduce bloat in the following mathematics
    let one = N::one();
    let three = one + one + one;

    let input_count = N::from_usize(inputs.len()).unwrap();

    let num_samples = inputs
        .iter()
        .map(|sample| N::from_usize(sample.len()).unwrap())
        .collect::<Array1<_>>();
    let total_num_samples = num_samples.sum();

    let variances = inputs
        .iter()
        .map(|sample| sample.var(one))
        .collect::<Array1<_>>();

    let spsq = (num_samples.mapv(|each| each - one) * &variances).sum()
        / (total_num_samples - input_count);

    let numer = (total_num_samples - input_count) * <N as Float>::ln(spsq)
        - (num_samples.mapv(|each| each - one) * variances.mapv(|each| <N as Float>::ln(each)))
            .sum();

    let denom = one
        + one / (three * (input_count - one))
            * (num_samples.mapv(|each| one / (each - one)).sum()
                - one / (total_num_samples - input_count));

    let statistic = numer / denom;

    let chi2 =
        statrs::distribution::Chi::new(<N as ToPrimitive>::to_f64(&(input_count - one)).unwrap())?;

    // This sqrt is due to a bug (i think) in statrs, they pass to the gamma as x^2 / 2, rather
    // than x / 2. By taking a square root here we get bach to x / 2
    let pvalue = <N as FromPrimitive>::from_f64(
        chi2.sf(<N as ToPrimitive>::to_f64(&statistic).unwrap().sqrt()),
    )
    .unwrap();

    Ok(BartlettTestResult { statistic, pvalue })
}
