def bar4(x):
    for i in range(10):
        if x:  # [no-else-continue]
            continue
        try:
            return
        except ValueError:
            continue
