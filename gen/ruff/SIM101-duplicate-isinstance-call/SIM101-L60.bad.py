isinstance(f"{(lambda: 0)}", int) or isinstance(f"{(lambda: 0)}", str)
isinstance(f"{0:{(lambda: 0)}}", int) or isinstance(f"{0:{(lambda: 0)}}", str)
isinstance(f"{0:\x22}", int) or isinstance(f"{0:\x22}", str)
isinstance(f"{0:\x7b}", int) or isinstance(f"{0:\x7b}", str)

# Regression test for: https://github.com/astral-sh/ruff/pull/25061
