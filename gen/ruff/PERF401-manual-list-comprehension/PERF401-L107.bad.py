def f():
    if True:
        for i in range(10):  # single-line comment 1 should be protected
            # single-line comment 2 should be protected
            if i % 2:  # single-line comment 3 should be protected
                result.append(i)  # PERF401
