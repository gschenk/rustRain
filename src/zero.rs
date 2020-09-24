// This module provides some functions to determine if a real number
// is within an epsilon environment of zero. For real numbers one
// cannot assume equality, only proximity.

// The factors between these parameters may be chosen for a
// compromise of precission and numerical stability.
pub const EPSILON: f64 = 128.0 * f64::EPSILON;
const RELTH: f64 = 16.0 * f64::EPSILON;
const TOL: f64 = 64.0; // Tolerance factor in approximate comparision

// reliably compares if two float numbers are equal
// https://stackoverflow.com/a/32334103/3842889
fn equal(a: f64, b: f64, epsilon: f64, relth: f64) -> bool {
    if a == b {
        return true;
    };

    let diff = (a - b).abs();
    let norm = (a.abs() + b.abs()).min(f64::MAX);
    return diff < (epsilon * norm).max(relth);
}

// reliably compares if two float numbers are equal
// https://stackoverflow.com/a/32334103/3842889
pub fn f64equal(a: f64, b: f64) -> bool {
    return equal(a, b, EPSILON, RELTH);
}

// compare if two vectors Vec<f64> are equal
fn vectors(av: &Vec<f64>, bv: &Vec<f64>, epsilon: f64, relth: f64) -> bool {
    // both are empty, as a definition: same
    if av.is_empty() && bv.is_empty() {
        return true;
    }

    // sizes must be equal
    if av.len() != bv.len() {
        return false;
    }

    // each element must be equal
    return av
        .iter()
        .zip(bv.iter())
        .map(|(&a, &b)| equal(a, b, epsilon, relth))
        .fold(true, |acc, x| acc && x);
}

pub fn vecf64equal(av: &Vec<f64>, bv: &Vec<f64>) -> bool {
    return vectors(av, bv, EPSILON, RELTH);
}

pub fn vecf64similar(av: &Vec<f64>, bv: &Vec<f64>) -> bool {
    return vectors(av, bv, TOL * EPSILON, TOL * RELTH);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_floats() {
        let a = 1 as f64;
        let b = 1.0 + EPSILON;
        let c = 1.0 + 1e-13; // this value is not equal to a, b
        let d = 1.0 - 1e-13; // this value is not equal to a, b
        assert!(f64equal(f64::MIN_POSITIVE, 0.0));
        assert!(f64equal(a, b));
        assert!(!f64equal(a, c));
        assert!(!f64equal(a, d));
    }

    #[test]
    fn equal_float_vecs() {
        let v0: Vec<f64> = vec![];
        let va = vec![1.0, 1.0, 1.0];
        let vb = vec![1.0, 1.0];
        let vc = vec![1.0, 1.1e-23, 4.342e9];
        let vd = vec![1.0, 0.0, 4.342e9];
        assert!(vecf64equal(&v0, &v0));
        assert!(vecf64equal(&va, &va));
        assert!(vecf64equal(&vc, &vc));
        assert!(!vecf64equal(&v0, &va));
        assert!(!vecf64equal(&va, &vb));
        assert!(vecf64equal(&vc, &vd));
    }
}
