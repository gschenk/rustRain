pub mod input;

// Problem collects data and characterises problem
#[derive(Debug)]
pub struct Problem {
    pub water_0: f64,      //initial water level on each segment
    pub grounds: Vec<f64>, //ground level
    pub water_tot: u64,    // total amount of water, conserved value!
    groundsize: u64,
    ground_min: u64,
    pub ground_max: u64,
    ground_vol: u64,
    saturation_water: u64,
}

impl Problem {
    pub fn new(duration: u64, profile: &[u64]) -> Problem {
        // convert ground and get some properties
        let grounds: Vec<f64> = profile.iter().map(|x| *x as f64).rev().collect();
        let ground_min = profile.iter().min().unwrap().clone();
        let ground_max = profile.iter().max().unwrap().clone();
        let groundsize = grounds.len() as u64;
        let ground_vol = profile.iter().sum();

        let water_0 = duration as f64;
        let water_tot = (water_0 as u64) * groundsize;

        // amount of water to fills all wells level with the highest peak
        let saturation_water = groundsize * ground_max - ground_vol;

        Problem {
            water_0,
            grounds,
            water_tot,
            groundsize,
            ground_min,
            ground_max,
            ground_vol,
            saturation_water,
        }
    }
}

pub mod solve {
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
}

// reliably compares if two float numbers are equal
// https://stackoverflow.com/a/32334103/3842889
pub fn f64equal(a: f64, b: f64) -> bool {
    const TOLERANCE: u64 = 128;
    const EPS: f64 = f64::EPSILON * TOLERANCE as f64;

    if a == b {
        return true;
    };

    let diff = (a - b).abs();
    let norm = (a.abs() + b.abs()).min(f64::MAX);
    return diff < (EPS * norm).max(f64::MIN);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Clone)]
    struct Case(u64, Vec<u64>, Vec<f64>);

    // simple test cases, pen-and-paper results
    fn provide_cases(token: &str) -> Vec<Case> {
        let simple = vec![
            Case(
                0,
                vec![5, 5, 0, 0, 0, 0, 5, 5],
                vec![5.0, 5.0, 3.0, 3.0, 3.0, 3.0, 5.0, 5.0],
            ),
            Case(
                1,
                vec![5, 5, 0, 0, 0, 0, 5, 5],
                vec![5.0, 5.0, 3.0, 3.0, 3.0, 3.0, 5.0, 5.0],
            ),
            Case(
                2,
                vec![5, 5, 0, 0, 0, 0, 5, 5],
                vec![5.0, 5.0, 4.0, 4.0, 4.0, 4.0, 5.0, 5.0],
            ),
        ];
        let saturation = vec![
            Case(1, vec![2, 0, 0, 2], vec![2.0, 2.0, 2.0, 2.0]),
            Case(
                2,
                vec![4, 4, 0, 0, 0, 0, 4, 4],
                vec![4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0],
            ),
        ];
        let oversaturation = vec![Case(
            3,
            vec![5, 5, 0, 0, 0, 0, 5, 5],
            vec![5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5, 5.5],
        )];
        let symmetry = vec![
            Case(
                1,
                vec![5, 4, 3, 0, 0, 0],
                vec![5.0, 4.0, 3.0, 2.0, 2.0, 2.0],
            ),
            Case(
                1,
                vec![0, 0, 0, 4, 3, 5],
                vec![2.0, 2.0, 2.0, 3.0, 4.0, 5.0],
            ),
            Case(2, vec![7, 6, 5, 0, 0], vec![7.0, 6.0, 5.5, 5.5, 5.5]),
            Case(2, vec![0, 0, 5, 6, 7], vec![5.5, 5.5, 5.5, 6.0, 7.0]),
        ];
        let watersheds = vec![
            Case(1, vec![0, 3, 0], vec![1.5, 3.0, 1.5]),
            Case(1, vec![0, 3, 3, 0], vec![2.0, 3.0, 3.0, 2.0]),
            Case(
                1,
                vec![6, 0, 4, 4, 0, 6],
                vec![6.0, 3.0, 4.0, 4.0, 3.0, 6.0],
            ),
            Case(
                2,
                vec![0, 5, 0, 5, 0, 5, 0],
                vec![3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0],
            ),
            Case(
                2,
                vec![0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0],
                vec![
                    3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0,
                ],
            ),
            Case(1, vec![0, 3, 4, 3, 0], vec![2.5, 3.0, 3.0, 2.5]),
        ];
        if token == "saturation" {
            return saturation;
        }
        if token == "all" {
            return [
                simple.as_slice(),
                saturation.as_slice(),
                oversaturation.as_slice(),
                symmetry.as_slice(),
                watersheds.as_slice(),
            ]
            .concat();
        }
        return vec![];
    }

    #[test]
    fn saturation() {
        let cases = provide_cases("saturation");
        for case in cases.iter() {
            let Case(a, b, _) = case;
            let problem = Problem::new(*a, &b);

            // expect that water saturation and total water are equal
            assert_eq!(problem.saturation_water, problem.water_tot);
        }
    }

    #[test]
    fn equal_floats() {
        let a = 1 as f64;
        let b = 1.0 + f64::EPSILON;
        let c = 1.0 + 1e-13; // this value is not equal to a, b
        let d = 1.0 - 1e-13; // this value is not equal to a, b
        assert_eq!(f64equal(a, b), true);
        assert_ne!(f64equal(a, c), true);
        assert_ne!(f64equal(a, d), true);
    }

    #[test]
    fn solutions() {
        let a_vec: Vec<f64> = vec![3.0, 2.0, 2.0];
        let bs: Vec<f64> = vec![3.0, 1.0, 0.0];
        let expected_1: Vec<f64> = vec![0.0, 1.0, 2.0];
        let expected_2: f64 = 3.0;
        let received = solve::Solution::new(a_vec, &bs);
        assert_eq!(expected_1, received.water_covers);
        assert_eq!(expected_2, received.water_tot);
    }
}
