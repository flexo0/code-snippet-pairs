def foo1(x, y, z):
    if x:  # [no-else-return]
        a = 1
        return y
    b = 2
    return z
