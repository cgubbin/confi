# confidence
---

A simple crate for dealing with confidence levels, significance levels and confidence intervals.

## Significance Levels
---
The significance level describes the evidence present in a sample under test which allows for rejection of the null hypothesis (the state when two outcomes or possibilities are the same).
In a measurement model the null hypothesis under test is typically whether a given reading is indistinguishable from another, or whether it is indistinguishable from the system response at zero stimulus (the minimum detectable value).

```rust
use confi::SignificanceLevel;

let from_fraction = SignificanceLevel::fractional(0.1).unwrap();
let from_percentage = SignificanceLevel::percentage(10.0).unwrap();

assert_eq!(from_fraction, from_percentage);

```


## Confidence Levels
---
The confidence level describes the probability of obtaining the same result in repeated data collection processes.
```rust
use confi::ConfidenceLevel;

let from_fraction = ConfidenceLevel::fractional(0.1).unwrap();
let from_percentage = ConfidenceLevel::percentage(10.0).unwrap();

assert_eq!(from_fraction, from_percentage);
```

## Confidence intervals
---
A confidence interval is a range of results which can be deemed to contain a measured value to a given confidence level.
```rust
use confi::{ConfidenceLevel, ConfidenceInterval, Confidence};

let from_fraction = ConfidenceLevel::fractional(0.1).unwrap();
let interval = ConfidenceInterval::new(1.0..=3.0, from_fraction);

assert!(interval.contains(2.0));

```
