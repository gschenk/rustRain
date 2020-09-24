// Solver for simple nearly linear equations. By itterative approximation.
// Use this only for the most harmless, nearly linear, equations.
use crate::zero;

// Solver Parameters:
// adaptive step-size constant parameters
const TOL: f64 = 4.0 * zero::EPSILON;
const MIN_STEP: f64 = TOL / 4.0;
const MAX_STEP: f64 = 0.1;

// keep this number low, if the solver cannot finish in about 100
// iterations more wont do good. Instead take a proper ODE solver.
const MAX_ITERATIONS: usize = 1000;

// boost step size higher = faster approach, smaller < safer convergence
const BOOST: f64 = 3.0;

// Assymetric step size for lowering/raising to avoid wobble around zero
const ASSY_FACTOR: f64 = 0.9 / BOOST;

// Toggle to let solver fail quietly and return value instead of panicing
const QUIET: bool = false;
const DEBUG: bool = false; // debou output

// step_size returns the size of the next step
fn step_size(delta: f64) -> f64 {
    return (BOOST * delta.abs()).max(MIN_STEP).min(MAX_STEP);
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

        if DEBUG && i > (MAX_ITERATIONS / 2) {
            println!("i, s, x, d {} {} {} {}", i, step_size(delta), x, delta)
        };
        // finish criterium
        if delta.abs() < TOL {
            break;
        }

        // panic when we are running out of iterations!
        if !QUIET && i + 1 >= MAX_ITERATIONS {
            panic!(
                "Solver reached max iterations: {}, delta: {}",
                MAX_ITERATIONS, delta
            )
        }

        // approach zero from either side and make a step
        if delta < 0.0 {
            x += step_size(delta);
            continue;
        } else {
            x -= ASSY_FACTOR * step_size(delta);
        }
    }
    return x;
}
