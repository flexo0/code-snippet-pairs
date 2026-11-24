def bar4(x):
    if x:  # [no-else-raise]
        raise Exception(True)
    try:
        raise Exception(False)
    except ValueError:
        raise Exception(None)


###
# Non-errors
###
