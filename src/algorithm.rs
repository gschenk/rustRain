// raise land algorithm
// we raise the land, by a lift value 'lift', in several steps. The first step
// ends when a saturation state is reached, where the water level is equal with
// the highest peak:
// v_underwater - v_displacement - v_water = 0
// Where v_underwater is the volume below the highest peak.  At this poin the
// problem is divided in sub problems.  For each subproblem a new highest peak
// is found and a new saturation level reached.

use crate::solutions::Solution;
use crate::Problem;
use std::iter::successors;

// Collector accumulates results and intermediate results from each
// recursion call
#[derive(Debug)]
struct Collector {
    segments: Vec<f64>,
}

impl Collector {
    // constructs a collector with segment's initial values
    fn new(size: usize) -> Self {
        let segments: Vec<f64> = successors(Some(0.0), |s| Some(*s)).take(size).collect();
        return Collector { segments };
    }
    fn set_level(&mut self, level: f64, i: usize) {
        self.segments[i] = level;
    }
}

#[derive(Debug)]
struct RecursorPars {
    water: f64,
    start: usize,
    end: usize,
    left_edge_peaks: usize,
    right_edge_peaks: usize,
    nest: u64,
}

impl RecursorPars {
    fn new(
        water: f64,
        start: usize,
        end: usize,
        left_edge_peaks: usize,
        right_edge_peaks: usize,
        nest: u64,
    ) -> Self {
        return Self {
            water,
            start,
            end,
            left_edge_peaks,
            right_edge_peaks,
            nest,
        };
    }
}

// raise initialises and calls the recursion function and pieces results together
pub fn raise(p: Problem) -> Solution {
    // initialize collector
    let collector0 = Collector::new(p.groundsize);

    let recursor_pars = RecursorPars::new(p.water_tot as f64, 0, p.groundsize - 1, 0, 0, 0);
    let collector = recursor(recursor_pars, &p.grounds, collector0);

    let Collector { segments } = collector;

    let levels: Vec<f64> = segments.iter().map(|s| *s).collect();

    // calculate the water levels in reverse, starting left going right
    let rev_pars = RecursorPars::new(p.water_tot as f64, 0, p.groundsize - 1, 0, 0, 0);
    let rev_grounds: Vec<u64> = p.grounds.clone().iter().rev().map(|a| *a).collect();
    let rev_coll0 = Collector::new(p.groundsize);
    let rev_collector = recursor(rev_pars, &rev_grounds, rev_coll0);

    // then average results of both calculations
    let average_levels: Vec<f64> = rev_collector
        .segments
        .iter()
        .rev()
        .zip(levels)
        .map(|(a, b)| (a + b) / 2.0)
        .collect();
    return Solution::new(average_levels, &p.grounds);
}

// well_volume calculates volume of a well from its ground semgments and height
fn well_volume(gs: &[u64], heigth: u64) -> f64 {
    let volume = heigth as f64 * gs.len() as f64;
    let land: f64 = gs.iter().map(|g| *g as f64).sum();
    return volume - land;
}

struct WaterDistribution {
    left: f64,
    right: f64,
}

// water_distribution function
// distributes water like rain: evenly by area. As long as there is enough room
// on both sides to take in water.  When one side reaches saturation it
// distributes water by accounting for displacment by submerged land mass.
fn water_distribution(
    water: f64,
    has_left: bool,
    has_right: bool,
    peak_heigth: u64,
    peak_width: f64,
    left_grounds: &[u64],
    right_grounds: &[u64],
    at_left_edge: bool,
    at_right_edge: bool,
    left_edge_peaks: usize,
    right_edge_peaks: usize,
) -> WaterDistribution {
    //trivial cases
    if !has_left {
        return WaterDistribution {
            left: 0.0,
            right: water,
        };
    }
    if !has_right {
        return WaterDistribution {
            left: water,
            right: 0.0,
        };
    }

    // If both ranges have parts that are still submerged displacment is not limiting water
    // uptake. Water is distributed by ranges only, that represents the area it rains upon.
    // The peak(s) separating left and right ranges are distributed evenly.
    let f_dist_range = |gs: &[u64]| peak_width / 2.0 + gs.len() as f64;

    // distribute water from peaks evenly to either side, and correct for boundary effects:
    // (The problem has impermeable boundaries. When these are next to the ranges considered
    // here they behave differently than other tiles.)
    let left_range = if !at_left_edge {
        f_dist_range(left_grounds) + 0.5f64.max(left_edge_peaks as f64)
    } else {
        f_dist_range(left_grounds)
    };
    let right_range = if !at_right_edge {
        f_dist_range(right_grounds) + 0.5f64.max(right_edge_peaks as f64)
    } else {
        f_dist_range(right_grounds)
    };

    let f_rain = |r| r * water / (left_range + right_range);
    let left_rain = f_rain(left_range);
    let right_rain = f_rain(right_range);

    // check if well has enough space to hold water
    let left_well_volume = well_volume(left_grounds, peak_heigth);
    let right_well_volume = well_volume(right_grounds, peak_heigth);

    // if either side has not enough space to hold rain, distribute excees to the other side
    let mut left = left_rain;
    let mut right = right_rain;
    if left_rain > left_well_volume {
        left = left_well_volume;
        right = water - left;
    } else if right_rain > right_well_volume {
        right = right_well_volume;
        left = water - right;
    }

    return WaterDistribution { left, right };
}

fn recursor(pars: RecursorPars, grounds: &[u64], mut collector: Collector) -> Collector {
    // destructure parameters
    let RecursorPars {
        water,
        start,
        end,
        left_edge_peaks,
        right_edge_peaks,
        nest,
    } = pars;

    // find the highest peak
    let peak_heigth = grounds.iter().max().unwrap();

    // find position of peak in list
    let i_peak: usize = grounds.iter().position(|x| x == peak_heigth).unwrap();
    let absolute_peak = i_peak + start; // absoulte position of peak in collector vector

    // see if adjacent segments right of the present one are at the same level
    let n_adjacent_peaks = grounds[i_peak..]
        .iter()
        .take_while(|g| g == &peak_heigth)
        .count();

    // check if peak is at extremes of our range
    let has_left = i_peak != 0;

    // consider adjacent peaks for the right one
    let has_right = i_peak + n_adjacent_peaks != grounds.len();

    // peaks at the left boundary are a special condition for water distribution
    // one recursion level down
    let new_left_edge_peaks = if !has_left && n_adjacent_peaks > 0 {
        n_adjacent_peaks
    } else {
        0
    };
    let new_right_edge_peaks = if !has_right && n_adjacent_peaks > 0 {
        n_adjacent_peaks
    } else {
        0
    };

    // determine if peaks are under water
    let displace = |gs: &[u64]| -> f64 { gs.iter().map(|g| *g as f64).sum() };
    let underwater: bool = displace(grounds) + water > *peak_heigth as f64 * grounds.len() as f64;

    // we are already done with this peak and its adjacent neighbours and
    // can add it to collector
    if underwater {
        return collector;
    } else {
        for i in 0..n_adjacent_peaks {
            collector.set_level(grounds[i + i_peak] as f64, i + absolute_peak);
        }
    }

    // grounds left and right of peak
    let grounds_left = &grounds[..i_peak];
    let grounds_right = if has_right {
        &grounds[i_peak + n_adjacent_peaks..]
    } else {
        &[]
    };

    // determine if the present range is ajacent to the right edge
    let at_left_edge: bool = start == 0;
    let at_right_edge: bool = end == collector.segments.len() - 1;

    let WaterDistribution {
        left: water_left,
        right: water_right,
    } = water_distribution(
        water,
        has_left,
        has_right,
        *peak_heigth,
        n_adjacent_peaks as f64,
        &grounds_left,
        &grounds_right,
        at_left_edge,
        at_right_edge,
        left_edge_peaks,
        right_edge_peaks,
    );

    // try something here:
    let new_level_left = (water_left + displace(grounds_left)) / grounds_left.len() as f64;
    let new_level_right = (water_right + displace(grounds_right)) / grounds_right.len() as f64;

    // check if there is world left left of the present peak
    if has_left {
        let grounds_left = &grounds[..i_peak];
        let end_left = absolute_peak - 1;

        // set water level to these segments
        for i in start..end_left + 1 {
            collector.set_level(new_level_left, i);
            //println!("{:?}", collector.segments[i]);
        }

        // going into left recursion, not a tail call, but for most terrains
        // this is much rarer than right recursions
        let left_pars = RecursorPars::new(
            water_left,
            start,
            end_left,
            new_left_edge_peaks,
            new_right_edge_peaks,
            nest + 1,
        );
        collector = recursor(left_pars, &grounds_left, collector);
    }

    // END OF RECURSION
    // if the present peak is already on the rightmost segment, we are done here
    if absolute_peak + n_adjacent_peaks - 1 == end {
        return collector;
    }

    // going right, after first peak
    let start_right = absolute_peak + n_adjacent_peaks;

    // set present water level for segments right of peak
    for i in start_right..end + 1 {
        collector.set_level(new_level_right, i);
    }

    let right_pars = RecursorPars::new(
        water_right,
        start_right,
        end,
        new_left_edge_peaks,
        new_right_edge_peaks,
        nest + 1,
    );

    // Tail Call  It would be quite interesting to know if tail call optimization works for
    // this function. It seems to be quite a difficult topic in Rust.
    return recursor(right_pars, &grounds_right, collector);
}
