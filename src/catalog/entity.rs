use num::{Bounded, Num};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Range<T: num::Num> {
    pub min: T,
    pub q25: T,
    pub q50: T,
    pub q75: T,
    pub max: T,
}

pub fn get_range<T: Ord + Num + Bounded + Copy>(values: &mut [T]) -> Range<T> {
    let len = values.len();
    if len == 0 {
        tracing::error!("Creating range from zero len array, it is possibly a bug, returning a invalid range with all zeros");
        return Range {
            min: T::zero(),
            q25: T::zero(),
            q50: T::zero(),
            q75: T::zero(),
            max: T::zero(),
        };
    }
    values.sort();
    // we needa clone the quantiles
    // because we need the values being stored as Range<T>
    // while all of them should be remaining in the vector
    let min = *values.iter().min().unwrap_or(&T::zero());
    let max = *values.iter().max().unwrap_or(&T::max_value());
    let q25 = *values.get(len / 4).unwrap_or(&T::zero());
    let q50 = *values.get(len / 2).unwrap_or(&T::zero());
    let q75 = *values.get(len * 3 / 4).unwrap_or(&T::zero());
    Range {
        min,
        q25,
        q50,
        q75,
        max,
    }
}
