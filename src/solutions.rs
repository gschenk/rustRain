// module solve provides structures to store results and functions to
// calculate the equilibrium state of water
use super::{f64equal, Problem};
mod algorithm;

// Solution stores results
// levels are the overal levels of water or dry land per segment,
// which ever is on top
// water covers is only the amount of water upon the the land
// water_tot is the overal amount of water, for plausibility checks
#[derive(Debug)]
pub struct Solution {
    pub levels: Vec<f64>,
    pub water_covers: Vec<f64>,
    pub water_tot: f64,
}

impl Solution {
    // arguments: levels: a vector of ground/water levels
    // grounds: slice of bare grounds
    fn new(levels: Vec<f64>, grounds: &[u64]) -> Solution {
        let water_covers: Vec<f64> = levels
            .iter()
            .zip(grounds.iter())
            .map(|(&a, &b)| a - b as f64)
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
pub fn select_fn(problem: &Problem) -> Box<dyn Fn(Problem) -> Solution> {
    // zero days of rain
    if f64equal(problem.water_0, 0.0) {
        return Box::new(|x| dry(x));
    }

    // flat world profile
    if problem.ground_max == problem.ground_min {
        return Box::new(|x| flat(x));
    }

    // saturation, water level equal to highest land
    if problem.water_tot == problem.saturation_water {
        return Box::new(|x| saturation(x));
    }

    // land is entirely under water
    if problem.water_tot > problem.saturation_water {
        return Box::new(|x| full(x));
    }

    // function for general case
    return Box::new(|x| algorithm::raise(x));
}

// all solver functions must have the same signature:
// fn f(p: Problem) -> Solution { ... }

// trivial solver for a dry world
fn dry(p: Problem) -> Solution {
    let levels = p.grounds.iter().map(|x| *x as f64).collect();
    return Solution::new(levels, &p.grounds);
}

// trivial solver for a flat world
fn flat(p: Problem) -> Solution {
    let levels = p.grounds.iter().map(|&x| x as f64 + p.water_0).collect();
    return Solution::new(levels, &p.grounds);
}

// saturation() the world is filled up to the level of highest ground
fn saturation(p: Problem) -> Solution {
    let levels = vec![p.ground_max as f64; p.grounds.len()];
    return Solution::new(levels, &p.grounds);
}

// full: the world is filled above saturation
fn full(p: Problem) -> Solution {
    let water_extra = (p.water_tot - p.saturation_water) as f64;
    let level = p.ground_max as f64 + water_extra / p.groundsize as f64;
    let levels = vec![level; p.grounds.len()];
    return Solution::new(levels, &p.grounds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solutions_struct() {
        let a_vec: Vec<f64> = vec![3.0, 2.0, 2.0];
        let bs: Vec<u64> = vec![3, 1, 0];
        let expected_1: Vec<f64> = vec![0.0, 1.0, 2.0];
        let expected_2: f64 = 3.0;
        let received = Solution::new(a_vec, &bs);
        assert_eq!(expected_1, received.water_covers);
        assert_eq!(expected_2, received.water_tot);
    }
}
