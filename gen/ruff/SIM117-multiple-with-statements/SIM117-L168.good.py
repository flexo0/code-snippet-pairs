with A() as a:
    async with B() as b, C() as c:
        print(a, b, c)

# SIM117 — nested sync with under an async with parent.
