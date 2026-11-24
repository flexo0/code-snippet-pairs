even_nums = {2 * x for x in range(3)}
odd_nums = {
    2 * x + 1 for x in range(3)
}
small_nums = f"{ {a if a < 6 else 0 for a in range(3)} }"
