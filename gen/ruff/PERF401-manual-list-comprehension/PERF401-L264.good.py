def f():
    values = [1, 2, 3]
    # this should be replaced with a comprehension
    result = [i + 1 for i in values]  # PERF401
