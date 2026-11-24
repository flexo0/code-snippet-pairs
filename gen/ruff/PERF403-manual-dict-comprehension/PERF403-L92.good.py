def foo():
    result = {name: idx for idx, name in enumerate(fruit)}  # PERF403
