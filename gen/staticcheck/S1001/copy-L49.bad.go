package p

func fn2() {
	for i, v := range src { //@ diag(`should use copy`)
		dst.b[i] = v
	}
}
