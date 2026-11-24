if result.eofs == "O":
    pass
elif result.eofs == "S":
    skipped = 1
elif result.eofs == "F":
    errors = 1
elif result.eofs == "E":
    errors = 1
elif result.eofs == "X":
    errors = 1
elif result.eofs == "C":
    errors = 1


# OK
