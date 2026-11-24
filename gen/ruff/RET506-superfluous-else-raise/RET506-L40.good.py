def foo3(x, y, z):
    if x:
        a = 1
        if y:  # [no-else-raise]
            b = 2
            raise Exception(y)
        c = 3
        raise Exception(x)
    d = 4
    raise Exception(z)
