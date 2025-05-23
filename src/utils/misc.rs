use std::ops::RangeInclusive;

pub fn float_interpolate(x: f64, min_x: f64, max_x: f64, min_value: f64, max_value: f64) -> f64 {
    if min_x == max_x {
        return min_value;
    }

    let factor = (x - min_x) / (max_x - min_x);
    min_value + factor * (max_value - min_value)
}

pub fn clamp_range<T: Copy + PartialOrd>(value: &mut T, range: &RangeInclusive<T>) {
    if *value < *range.start() {
        *value = *range.start();
    } else if *value > *range.end() {
        *value = *range.end();
    }
}
