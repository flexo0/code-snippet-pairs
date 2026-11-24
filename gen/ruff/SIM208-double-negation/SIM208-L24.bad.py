print(not not x in y)  # SIM208
print(not not x not in y)  # SIM208
print(not not x is y)  # SIM208
print(not not x is not y)  # SIM208
print(not not bool(x))  # SIM208
print(not not x in y in z)  # SIM208

# These may not be `bool`, so the fix still wraps them in `bool(...)`.
