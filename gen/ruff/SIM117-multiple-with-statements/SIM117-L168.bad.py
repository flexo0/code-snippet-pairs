with A() as a:
    async with B() as b:
        async with C() as c:
            print(a, b, c)

# SIM117 — nested sync with under an async with parent.
