def crossover(a, b, rnd):
    g = copy(a)
    for c in TUNED:
        if rnd.random() < 0.5:
            g["weights"][c] = b["weights"][c]
    if rnd.random() < 0.5:
        g["model"], g["factor"] = b["model"], b["factor"]
    for k in ("max_depth", "step", "max", "astep", "amax"):
        if rnd.random() < 0.5:
            g[k] = b[k]
    g["conditions"] = [x if rnd.random() < 0.5 else y for x, y in zip(a["conditions"], b["conditions"])]
    return g
