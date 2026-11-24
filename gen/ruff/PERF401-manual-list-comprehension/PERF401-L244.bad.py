def f():
    def g():
        for a in values:
            result.append(a + 1)  # PERF401
