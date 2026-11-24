def f():
    def g():
        result.extend(a + 1 for a in values)  # PERF401
