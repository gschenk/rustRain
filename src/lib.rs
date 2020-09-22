pub mod input;

// Problem collects data and characterises problem
#[derive(Debug)]
pub struct Problem {
    pub water_0: f64,      //initial water level on each segment
    pub grounds: Vec<f64>, //ground level
    pub water_tot: f64,    // total amount of water, conserved value!
    ground_min: f64,
    pub ground_max: f64,
    ground_vol: f64,
    saturation_water: f64,
}

impl Problem {
    pub fn new(duration: u64, profile: &[u64]) -> Problem {
        // convert ground and get some properties
        let grounds: Vec<f64> = profile.iter().map(|x| *x as f64).rev().collect();
        let ground_min = profile.iter().min().unwrap().clone() as f64;
        let ground_max = profile.iter().max().unwrap().clone() as f64;
        let groundsize = grounds.len() as f64;
        let ground_vol = grounds.iter().sum();

        let water_0 = duration as f64;
        let water_tot = water_0 * groundsize;

        // amount of water to fills all wells level with the highest peak
        let saturation_water = groundsize * ground_max - ground_vol;

        Problem {
            water_0,
            grounds,
            water_tot,
            ground_min,
            ground_max,
            ground_vol,
            saturation_water,
        }
    }
}
