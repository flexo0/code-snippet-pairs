package p

func example(x, y []int) []int {
	for _, e := range y {
		x = append(x, e)
	}
	return x
}
