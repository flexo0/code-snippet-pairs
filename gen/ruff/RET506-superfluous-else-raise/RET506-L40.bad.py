def foo3(x, y, z):
    if x:
        a = 1
        if y:  # [no-else-raise]
            b = 2
            raise Exception(y)
        else:
            c = 3
            raise Exception(x)
    else:
        d = 4
        raise Exception(z)
