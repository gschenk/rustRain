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
- For a certain class of problems water is distributed incorrectly.
  A blunt way to fix this is to run the same problem in reverse and
  average results.

- There is no proper solver to minimize the levelling function implemented.
  The present one is just a placeholder for prototyping the algorithm.

- The levelling function has bad properties and leads to stiff problem. In
  particular the discontinuity at `x > 0` has to be fixed to allow eg golden
  section method solver. Such a solver will be tolerant to stiffness due to
  inherent discontinuity of the function.

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


### Raise Land Algorithm (chosen algorithm)
- start with water level `l_i = d` for all _i_
- (fn 1) identify rightmost highest peak(s) with height `r_max`
- raise land until highest point `r_max = d` thus `r_i = p_i - p_max + d` 
- increase water level by water displacement `l = d + sum(r_i)/N` where
  `l_i = l` for all `i`
- incrementally increase water level with adaptive step size until
  `l - r_max <= EPSILON` (that's a real first watershed moment)
- distribute water between segments left, and right of the peak. For sufficiently
 raised sub-ranges the maximum amount of water that can be taken is determined
 by water displacement of sub-surface land. If land is not fully raised
 distribute by area. **This does not work right in certain cases**
- for `m` peaks with height `r_i = l_i` we get `m+1` independent problems.
  Recursion of (fn 1)
- return from (fn 1) when no peaks are left, fully raise all segments in sub
  problem `r_i = p_i`


#### Notes
Advantage of this algorithm is that no water is distributed and the problem of
re-distributing water onto a set of segments where water levels are already in
equilibrium will not occur.

Recursion depth is also limited by the largest possible number of peaks and one
`(ceiling(N/2) - 1) + 1`.

### Distribute and Level Algorithm (not chosen)
- rain all _d_ units of water per segment in one event `l_i = p_i + d`
- identify watersheds `p_i > p_(i-1)` and `p_i > p_(i+1)`,
  watersheds may be plateaus: `p_(i-1) < p_i = p_(i+1) = ... = p_(i+m) > p_(i+m+1)`.
- for a watershed _i_ move _m*d/2_ units of water to _i-1_
- iterate through all segments `i = 0 ... N-1` and run level algorithm right
  starting at `j=i` and incrementing `k = j+1`
- iterate through all segments `i = N ... 1` and run level algorithm left 
  starting at `j=i` and incrementing `k = j-1`
- check local equilibrium, every segment in a well has the same water level,
  if not run left and right levelling iterations again


#### Level Function
distinguish cases for adjacent segments j and k:
- `p_j < p_k` hit a wall:
  distribute water `l_i += w/m`, and return.
- `p_j <= l_k` in well, water is high
  collect water `w += d + p_k - p_i`, increment `m`, recursion at segment `k`
- `p_j > l_k - w` we found a sink
  dump all water `l_k += w` and return
- `p_j` > l_k` in well, water is low
  dump as much water as we can `w -= p_j - l_k` and `l_k = p_j`,
  recursion at segment `k`

The level function levels the water level `l_i` for all segments in a well.
Hittin a wall signifies the end of the well. At each function call we collect
water `w` and distance `m` from segment `i`. This function returns an updated
water level vector _L_ `{l_i}`.

#### Notes
This algorithm recursively distributes all water to a level that is equal
to the ground of the first segment by recursion of the _level function_.
This is iterated to move over all segments to find deeper wells. We might
have dumped water from the first segment to the second, just to remove it
to an even deeper well in the next step.

Since this only goes down into wells that are right of the previous segment
the previous step has to be run in reverse.

Watersheds are dealt with by pushing half of their water to the left before
starting the level function calls. !! this does not work !!

There are fringe cases at the boundaries conceivable where the left iteration
leaves water levels that are higher than the respective boundary walls. Another
pass of the right algorithm is necessary in that case.

#### What breaks this Algorithm:
Watersheds may be peaks, plateaus, but also small wells between Plateaus.
Think of [0, 0, 0, 6, 5, 4, 6, 0, 0, 0]. It is difficult to determine when such
high well fills up. Especially when dealing with fractional water reaching it.

