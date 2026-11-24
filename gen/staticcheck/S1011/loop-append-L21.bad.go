package p

func fn1() {
	for i := range a2 { //@ diag(`should replace loop`)
		b2 = append(b2, a2[i])
	}
}
