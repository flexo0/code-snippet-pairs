sorted(x)
sorted(x, reverse=True)
sorted(x, key=lambda e: e, reverse=True)
sorted(x, reverse=False)
sorted(x, key=lambda e: e, reverse=False)
sorted(x, reverse=False, key=lambda e: e)
sorted(x, reverse=True)
sorted(x, reverse=not x)
sorted(x, reverse=x)

# Regression test for: https://github.com/astral-sh/ruff/issues/7289
