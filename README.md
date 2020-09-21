# rustRain
Fills a 1-d landscape with discrete height function with water.

One unit of water is added to each segment per time step.

Expects two input parameters
Duration _d_: positiv integer
Profile _P_: List of N positive integers

Returns water level as list to STDOUT

## definitions
- `P = p_0, ..., p_i, ..., p_N`
- global minimum `p_min = min(P)`
- global maximum `p_max = max(P)`
- _peak_ local maximum
- _watershed_ peak that distributes water to either side
- _well_ range of segments between two peaks
- saturation volume `v_sat = sum_i ( p_max - p_i )`

## brainstorming

### trivial and simple cases
- no rain d=0
- too much rain `d > p_max - p_min`
- saturation rain `d * N >= v_sat`
- level profile `p_i = p_j` for all `i, j < N`
- single well

### recursive diffusion
- simple, only look at adjacent segments
- fringe conditions when wells are about to overflow may cause long or infinite recursion

### unidirectional iterative diffusion, from both boundaries
- start at i=0, flow from i to i+1 only, stop at i >= N/2
- alternate with i=N start, flow from i to i-1 only, stop at i-1 <= N/2
- correctional pass required, identify wet segments, equalize water level
- Issue: dry field might be overlooked if it would be flooded in second pass

### identify wells
