def foo():
    for idx, name in enumerate(fruit):
        result[name] = idx  # PERF403
