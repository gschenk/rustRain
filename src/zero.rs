// This module provides some functions to determine if a real number
// is within an epsilon environment of zero. For real numbers one
// cannot assume equality, only proximity.
const TOLERANCE: u64 = 128;
pub const EPSILON: f64 = f64::EPSILON * TOLERANCE as f64;

// reliably compares if two float numbers are equal
// https://stackoverflow.com/a/32334103/3842889
pub fn f64equal(a: f64, b: f64) -> bool {
    if a == b {
        return true;
    };

    let diff = (a - b).abs();
    let norm = (a.abs() + b.abs()).min(f64::MAX);
    return diff < (EPSILON * norm).max(f64::MIN);
}

// compare if two vectors Vec<f64> are equal
pub fn vecf64equal(av: &Vec<f64>, bv: &Vec<f64>) -> bool {
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
        .map(|(&a, &b)| f64equal(a, b))
        .fold(true, |acc, x| acc && x);
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
        assert_eq!(f64equal(a, b), true);
        assert_ne!(f64equal(a, c), true);
        assert_ne!(f64equal(a, d), true);
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
        assert_eq!(vecf64equal(&v0, &va), false);
        assert_eq!(vecf64equal(&va, &vb), false);
        assert_eq!(vecf64equal(&vc, &vd), false);
    }
}
