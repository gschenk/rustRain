use super::{f64equal, Problem};

// Solution stores results
// levels are the overal levels of water or dry land per segment,
// which ever is on top
// water covers is only the amount of water upon the the land
// water_tot is the overal amount of water, for plausibility checks
pub struct Solution {
    pub levels: Vec<f64>,
    pub water_covers: Vec<f64>,
    pub water_tot: f64,
}

impl Solution {
    // arguments: levels: a vector of ground/water levels
    // grounds: slice of bare grounds
    pub fn new(levels: Vec<f64>, grounds: &[f64]) -> Solution {
        let water_covers: Vec<f64> = levels
            .iter()
            .zip(grounds.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        let water_tot = water_covers.iter().sum();
        Solution {
            levels,
            water_covers,
            water_tot,
        }
    }
}

// categorise problems to deal with trivial and simple problems
// returns strings as placeholder. Will return function.
pub fn categorise(problem: Problem) -> &'static str {
    if f64equal(problem.water_0, 0.0) {
        return "dry";
    }
    // casting to a u64 is harmless here as it is the result of
    // integer arithmetic
    if problem.water_tot == problem.saturation_water {
        return "saturation";
    }
    if problem.water_tot > problem.saturation_water {
        return "above_saturation";
    }
    if problem.ground_max == problem.ground_min {
        return "flat_ground";
    }

    // general problem
    return "general";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solutions() {
        let a_vec: Vec<f64> = vec![3.0, 2.0, 2.0];
        let bs: Vec<f64> = vec![3.0, 1.0, 0.0];
        let expected_1: Vec<f64> = vec![0.0, 1.0, 2.0];
        let expected_2: f64 = 3.0;
        let received = Solution::new(a_vec, &bs);
        assert_eq!(expected_1, received.water_covers);
        assert_eq!(expected_2, received.water_tot);
    }
}
