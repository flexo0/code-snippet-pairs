def f():
    values = [1, 2, 3]
    result = list()  # this should be replaced with a comprehension
    for i in values:
        result.append(i + 1)  # PERF401
