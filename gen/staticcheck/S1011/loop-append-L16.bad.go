package p

func fn1() {
	for _, v := range a { //@ diag(`should replace loop`)
		b = append(b, v)
	}
}
