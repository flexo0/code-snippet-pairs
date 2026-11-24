package p

func fn() {
	for i := range b3 { //@ diag(`should use copy`)
		b4[i] = b3[i]
	}
}
