print(bool(x in y))  # SIM208
print(bool(x not in y))  # SIM208
print(bool(x is y))  # SIM208
print(bool(x is not y))  # SIM208
print(bool(bool(x)))  # SIM208
print(bool(x in y in z))  # SIM208

# These may not be `bool`, so the fix still wraps them in `bool(...)`.
