list((2 * x for x in range(3)))
list(((2 * x for x in range(3))))
list((((2 * x for x in range(3)))))

# Account for trailing comma in fix
# See https://github.com/astral-sh/ruff/issues/15852
