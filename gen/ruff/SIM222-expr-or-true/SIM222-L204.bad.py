print(f"{x=}" or "bar")  # SIM222
(lambda: 1) or True  # SIM222
(i for i in range(1)) or "bar"  # SIM222

# https://github.com/astral-sh/ruff/issues/21136
