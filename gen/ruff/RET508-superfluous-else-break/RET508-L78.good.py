def bar4(x):
    for i in range(10):
        if x:  # [no-else-break]
            break
        try:
            return
        except ValueError:
            break


###
# Non-errors
###
