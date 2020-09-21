# rustRain
Fills a 1-d landscape with discrete height function with water.

One unit of water is added to each segment per time step.

Expects two input parameters
Duration _d_: positiv integer
Profile _P_: List of N+1 positive integers

Returns water level as list to STDOUT

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


### Distribute and Level Algorithm
- rain all _d_ units of water per segment in one event `l_i = p_i + d`
- identify watersheds `p_i > p_(i-1)` and `p_i > p_(i+1)` 
- for a watershed _i_ move _d/2_ units of water to _i-1_
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
starting the level function calls.

There are fringe cases at the boundaries conceivable where the left iteration
leaves water levels that are higher than the respective boundary walls. Another
pass of the right algorithm is necessary in that case.

### Raise Land Algorithm
- start with water level `l_i = d` for all _i_
- (fn 1) identify highest peak(s) with height `r_max`
- raise land until highest point `r_max = d` thus `r_i = p_i - p_max + d` 
- increase water level by water displacement `l = d + sum(r_i)/N` where
  `l_i = l` for all `i`
- incrementally increase water level with adaptive step size until
  `l - r_max <= EPSILON` (that's a real first watershed moment)
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
