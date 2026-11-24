sorted([1, 2, 3], reverse=not (False or True))
sorted([1, 2, 3], reverse=not (False or True))

# These fixes need to be parenthesized to avoid syntax errors and behavior
# changes.
# See https://github.com/astral-sh/ruff/issues/15789
