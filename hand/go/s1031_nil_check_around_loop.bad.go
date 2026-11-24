package p

func example(s []int) {
	if s != nil {
		for _, x := range s {
			use(x)
		}
	}
}
