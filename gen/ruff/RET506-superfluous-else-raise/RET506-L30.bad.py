def foo1(x, y, z):
    if x:  # [no-else-raise]
        a = 1
        raise Exception(y)
    else:
        b = 2
        raise Exception(z)
