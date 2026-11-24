list(sorted(x))
reversed(sorted(x))
reversed(sorted(x, key=lambda e: e))
reversed(sorted(x, reverse=True))
reversed(sorted(x, key=lambda e: e, reverse=True))
reversed(sorted(x, reverse=True, key=lambda e: e))
reversed(sorted(x, reverse=False))
reversed(sorted(x, reverse=x))
reversed(sorted(x, reverse=not x))

# Regression test for: https://github.com/astral-sh/ruff/issues/7289
