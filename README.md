# rustRain
Fills a 1-d landscape with discrete height function with water.

One unit of water is added to each segment per time step.


## Useage
> cargo run [myinput.toml]

The input file must have two fields:
duration _d_: positiv integer
profile _P_: List of N+1 positive integers

Example:
    duration = 1
    profile = [0, 1, 2, 3]

The program returns a list of final levels of water and land to STDOUT.


## Known Issues
- Data structures are often not passed in a good way. This leaves room for
  optimisations.

## definitions
- `P = p_0, ..., p_i, ..., p_N`
- global minimum `p_min = min(P)`
- global maximum `p_max = max(P)`
- _peak_ local maximum
- _watershed_ peak that distributes water to either side
- _well_ range of segments between two peaks
- saturation volume `v_sat = sum_i ( p_max - p_i )`
- _right_ in direction of increasing _i_
- _left_ in direction of decreasing _i_

## Algorithms

### trivial and simple cases
- no rain d=0
- too much rain `d > p_max - p_min`
- saturation rain `d * N >= v_sat`
- level profile `p_i = p_j` for all `i, j < N`


### Divide at Watershed Algorithm (chosen algorithm)
- (fn 1) identify rightmost highest peak(s) with height `r_max`
- check if adjacent peaks segments have same height, if yes add to peak
- define range left and right of peak
- distribute water based on relative size of left/right ranges [this step
  needs careful corrections for boundary effects]
- check if distributed water for a range exceeds its holding capacity, if yes
  distribute to other side.
- determine average water level in each range for sum of water + displacement by
  submerged ground
- return (fn 1): recursion into left problem (fn 1) with left range and left water as parameters
- tail return (fn 1): recursion into right problem (fn 1) with right range, water as parameter
- return result when peak is already submerged
- return result when peak is rightmost field in range
- average over two passes, one with reversed `profile` vector, then average results;
  this removes a boundary problem that distributes slightly more water to the right.

#### Notes
Recursion depth is limited by the largest possible number of peaks and one
`s = (ceiling(N/2) - 1) + 1`. Complexity is at its worst O[2^s].
