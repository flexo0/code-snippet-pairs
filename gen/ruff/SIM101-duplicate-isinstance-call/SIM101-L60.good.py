isinstance(f"{(lambda: 0)}", (int, str))
isinstance(f"{0:{(lambda: 0)}}", (int, str))
isinstance(f"{0:\x22}", (int, str))
isinstance(f"{0:\x7b}", (int, str))

# Regression test for: https://github.com/astral-sh/ruff/pull/25061
