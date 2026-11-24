def foo3(x, y, z):
    for i in x:
        if i < y:
            a = 1
            if z:  # [no-else-continue]
                b = 2
                continue
            else:
                c = 3
                continue
        else:
            d = 4
            continue
