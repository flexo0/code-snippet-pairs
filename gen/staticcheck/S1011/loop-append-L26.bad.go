package p

func fn1() {
	for i := range a3 { //@ diag(`should replace loop`)
		v := a3[i]
		b3 = append(b3, v)
	}
}
