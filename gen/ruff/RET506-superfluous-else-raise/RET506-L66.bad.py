def foo6(x, y):
    if x:
        if y:  # [no-else-raise]
            a = 4
            raise Exception(x)
        else:
            b = 2
    else:
        c = 3
