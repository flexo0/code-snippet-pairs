package p

func example(x, y []int) []int {
	x = append(x, y...)
	return x
}
