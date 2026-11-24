def foo3(x, y, z):
    for i in x:
        if i < y:
            a = 1
            if z:  # [no-else-continue]
                b = 2
                continue
            c = 3
            continue
        d = 4
        continue
