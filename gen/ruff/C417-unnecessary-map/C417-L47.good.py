(x for x in (y if y else z))
(x for x in (y if y else z))
(x for x in (x, y, z))

# See https://github.com/astral-sh/ruff/issues/14808
# The following should be Ok since
# named expressions are a syntax error inside comprehensions
