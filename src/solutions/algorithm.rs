// raise land algorithm
// we raise the land, by a lift value 'lift', in several steps. The first step
// ends when a saturation state is reached, where the water level is equal with
// the highest peak:
// v_underwater - v_displacement - v_water = 0
// Where v_underwater is the volume below the highest peak.  At this poin the
// problem is divided in sub problems.  For each subproblem a new highest peak
// is found and a new saturation level reached.

use super::{Problem, Solution};
use crate::solver;

// levelling_equation
fn levelling_equation(x: f64, water: f64, grounds: &[u64]) -> f64 {
    let grounds_size = grounds.len() as f64;

    // find level of the present peaks
    let grounds_max = *grounds.iter().max().unwrap() as f64;

    // calculates the volume containing water and ground
    let underwater = (x + grounds_max) * grounds_size;

    // calculates the displaced volume for a given lift value x
    let displace: f64 = grounds
        .iter()
        .map(move |g| x + *g as f64)
        .filter(|x| *x > 0.0)
        .sum();

    // returns normalized delta
    return (underwater - displace - water) / grounds_size;
}

// function call in super:
//return Box::new(|x| raise(x));

// all solver functions must have the same signature:
// fn f(p: Problem) -> Solution { ... }

pub fn raise(p: Problem) -> Solution {
    // initial lift value, it's negative until the end
    let lift: f64 = p.water_0 - p.ground_max as f64;

    // the equation f(x) to be solved in the iteration solver
    // here it is curried with a closure on its parameters
    let equation = |x| levelling_equation(x, p.water_tot as f64, &p.grounds);

    // running the actual solver
    let _new_lift = solver::iterative(lift, equation);

    let levels = vec![]; // TODO placeholder
    return Solution::new(levels, &p.grounds);
}
