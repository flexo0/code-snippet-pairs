_ = Decimal.from_float(float("-nan"))
_ = Decimal.from_float(float("\x2dnan"))
_ = Decimal.from_float(float("\N{HYPHEN-MINUS}nan"))

# See: https://github.com/astral-sh/ruff/issues/21257
# fixes must be safe
