def foo():
    for idx, name in enumerate(fruit):
        if idx % 2:
            result[idx] = name  # PERF403
