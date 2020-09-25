pub mod algorithm;
pub mod input;
pub mod solutions;
pub mod solver;
pub mod zero;

// Problem collects data and characterises problem
#[derive(Debug)]
pub struct Problem {
    pub water_0: f64,      //initial water level on each segment
    pub grounds: Vec<u64>, //ground level
    pub water_tot: u64,    // total amount of water, conserved value!
    groundsize: usize,
    ground_min: u64,
    pub ground_max: u64,
    ground_vol: u64,
    saturation_water: u64,
}

impl Problem {
    pub fn new(duration: u64, profile: &[u64]) -> Problem {
        // convert ground and get some properties
        let grounds: Vec<u64> = profile.to_vec();
        let ground_min = profile.iter().min().unwrap().clone();
        let ground_max = profile.iter().max().unwrap().clone();
        let groundsize = grounds.len();
        let ground_vol = profile.iter().sum();

        let water_0 = duration as f64;
        let water_tot = (water_0 as u64) * groundsize as u64;

        // amount of water to fills all wells level with the highest peak
        let saturation_water = groundsize as u64 * ground_max - ground_vol;

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

#[cfg(test)]
mod tests {

    use super::solutions;
    use super::*;
    use crate::zero::vecf64similar;

    #[derive(Clone, Debug)]
    struct Case(u64, Vec<u64>, Vec<f64>);

    // simple test cases, pen-and-paper results
    fn provide_cases(token: &str) -> Vec<Case> {
        let simple = vec![
            Case(
                0,
                vec![5, 5, 0, 0, 0, 0, 5, 5],
                vec![5.0, 5.0, 0.0, 0.0, 0.0, 0.0, 5.0, 5.0],
            ),
            Case(
                1,
                vec![5, 5, 0, 0, 0, 0, 5, 5],
                vec![5.0, 5.0, 2.0, 2.0, 2.0, 2.0, 5.0, 5.0],
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
                vec![0, 0, 0, 3, 4, 5],
                vec![2.0, 2.0, 2.0, 3.0, 4.0, 5.0],
            ),
            Case(2, vec![7, 6, 5, 0, 0], vec![7.0, 6.0, 5.0, 5.0, 5.0]),
            Case(2, vec![0, 0, 5, 6, 7], vec![5.0, 5.0, 5.0, 6.0, 7.0]),
        ];
        let watersheds = vec![
            Case(1, vec![0, 3, 0], vec![1.5, 3.0, 1.5]),
            Case(1, vec![0, 3, 3, 0], vec![2.0, 3.0, 3.0, 2.0]),
            Case(
                1,
                vec![6, 0, 4, 4, 0, 6],
                vec![6.0, 3.0, 4.0, 4.0, 3.0, 6.0],
            ),
            Case(1, vec![0, 3, 4, 3, 0], vec![2.5, 3.0, 4.0, 3.0, 2.5]),
        ];
        // these tests fail spectacularly!
        let known_bugs = vec![
            Case(
                2,
                vec![0, 5, 0, 5, 0, 5, 0],
                vec![3.0, 5.0, 4.0, 5.0, 4.0, 5.0, 3.0],
            ),
            Case(
                2,
                vec![0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 0],
                vec![
                    3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0, 5.0, 3.0,
                ],
            ),
        ];
        if token == "simple" {
            return saturation;
        }
        if token == "saturation" {
            return saturation;
        }
        if token == "known_bugs" {
            return known_bugs;
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
    fn solve_simple() {
        let cases = provide_cases("simple");
        for case in cases.iter() {
            let Case(a, b, expected) = case;
            let problem = Problem::new(*a, &b);
            let solver = solutions::select_fn(&problem);
            let received = solver(problem).levels;

            assert!(vecf64similar(&received, &expected));
        }
    }
    #[test]
    fn solve_saturation() {
        let cases = provide_cases("saturation");
        for case in cases.iter() {
            let Case(a, b, expected) = case;
            let problem = Problem::new(*a, &b);
            let solver = solutions::select_fn(&problem);
            let received = solver(problem).levels;

            assert!(vecf64similar(&received, &expected));
        }
    }

    #[test]
    fn solve_oversaturation() {
        let cases = provide_cases("oversaturation");
        for case in cases.iter() {
            let Case(a, b, expected) = case;
            let problem = Problem::new(*a, &b);
            let solver = solutions::select_fn(&problem);
            let received = solver(problem).levels;

            assert!(vecf64similar(&received, &expected));
        }
    }
    #[test]
    fn solve_all() {
        let cases = provide_cases("all");
        //assert!(false);
        for case in cases.iter() {
            let Case(a, b, expected) = case;
            let problem = Problem::new(*a, &b);
            let solver = solutions::select_fn(&problem);
            println!("P{:?}", &problem);
            println!("E{:?}", &expected);
            let received = solver(problem).levels;

            println!("R {:?}", &received);
            assert!(vecf64similar(&received, &expected));
        }
    }
}
