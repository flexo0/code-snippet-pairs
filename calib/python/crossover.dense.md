# crossover

`crossover(a, b, rnd) -> g`: `g = copy(a)`, then with `p := rnd.random() < 0.5`, a draw each:

- ∀ `c` ∈ `TUNED`: `p` → `g["weights"][c] = b["weights"][c]`;
- `p` → `g["model"], g["factor"] = b["model"], b["factor"]`;
- ∀ `k` ∈ `{max_depth, step, max, astep, amax}`: `p` → `g[k] = b[k]`;
- `g["conditions"][i] = p ? a["conditions"][i] : b["conditions"][i]`, ∀ `i`.

`return g`.
