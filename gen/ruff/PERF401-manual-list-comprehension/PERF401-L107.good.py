def f():
    if True:
        # single-line comment 1 should be protected
        # single-line comment 2 should be protected
        # single-line comment 3 should be protected
        result.extend(i for i in range(10) if i % 2)  # PERF401
