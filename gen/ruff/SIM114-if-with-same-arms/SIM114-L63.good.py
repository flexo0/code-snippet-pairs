if result.eofs == "O":
    pass
elif result.eofs == "S":
    skipped = 1
elif result.eofs == "F" or result.eofs == "E" or result.eofs == "X" or result.eofs == "C":
    errors = 1


# OK
