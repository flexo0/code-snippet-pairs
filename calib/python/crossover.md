# crossover

`crossover(a, b, rnd)` makes a child of two candidates `a` and `b`, with `rnd` as its random generator.

The child starts as a copy of `a`. Then each of these takes `b`'s value instead, with a chance of one half, a draw
of `rnd.random()` below `0.5`:

- each tuned category's weight in `weights` (the categories of `TUNED`), a draw each;
- the nesting's `model` and `factor`, together on one draw;
- each of `max_depth`, `step`, `max`, `astep` and `amax`, a draw each.

Its `conditions` are a new list: each condition is `a`'s at the same place when a draw is below one half, else
`b`'s.

It returns the child.
