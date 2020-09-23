// raise land algorithm
// we raise the land, by a lift value 'lift', in several steps. The first step
// ends when a saturation state is reached, where the water level is equal with
// the highest peak:
// v_underwater - v_displacement - v_water = 0
// Where v_underwater is the volume below the highest peak.  At this poin the
// problem is divided in sub problems.  For each subproblem a new highest peak
// is found and a new saturation level reached.

use super::{Problem, Solution};
use crate::zero::f64equal;

// Solver Parameters:
// adaptive step-size constant parameters
const TOL: f64 = 128f64 * f64::EPSILON; // TODO get this from f64equal after refactor
const MIN_STEP: f64 = TOL / 2.0;
const MAX_STEP: f64 = 2.0;

// keep this number low, if the solver cannot finish in about 100
// iterations more wont do good. Instead take a proper ODE solver.
const MAX_ITERATIONS: usize = 300;

// Assymetric step size for lowering/raising to avoid wobble around zero
const ASSY_FACTOR: f64 = 0.9;

// step_size returns the size of the next step
fn step_size(delta: f64) -> f64 {
    return delta.abs().max(MIN_STEP).min(MAX_STEP);
}

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

//step = delta.abs().max(TOL / 2.0).min(2.0);
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
    let _new_lift = iteration_solver(lift, equation);

    let levels = vec![]; // TODO placeholder
    return Solution::new(levels, &p.grounds);
}

// this solver takes solves the equation f(x) and solves it
// for f(x_f) = 0 by variation of x in steps. The step size
// is adapted to the difference of f(x) from zero.
// Use this only for the most harmless, nearly linear, equations.
fn iteration_solver<F>(x0: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    // init variable x the iteration will solve for
    let mut x = x0;

    for i in 0..MAX_ITERATIONS {
        let delta: f64 = f(x);

        // adaptive step size, equals error, with boundaries
        println!("i:{}, delta:{}", i, delta);

        // the end criterium
        if f64equal(delta, 0.0) {
            break;
        }

        // approach zero from either side
        if delta < 0.0 {
            x += step_size(delta);
            continue;
        } else {
            x -= ASSY_FACTOR * step_size(delta);
        }
    }
    return x;
}
