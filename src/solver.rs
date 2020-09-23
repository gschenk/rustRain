// Solver for simple nearly linear equations. By itterative approximation.
// Use this only for the most harmless, nearly linear, equations.
use crate::zero;

// Solver Parameters:
// adaptive step-size constant parameters
const TOL: f64 = zero::EPSILON;
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

// this solver takes solves the equation f(x) and solves it
// for f(x_f) = 0 by variation of x in steps. The step size
// is adapted to the difference of f(x) from zero.
// argument:
//    function f(x)
//    initial value x_0
pub fn iterative<F>(x0: f64, f: F) -> f64
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
        if zero::f64equal(delta, 0.0) {
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
