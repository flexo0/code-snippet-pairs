package p

func fn4() {
	for i := range b { //@ diag(`should use copy`)
		a1p[i] = b[i]
	}
	for i := range a1p { //@ diag(`should use copy`)
		b[i] = a1p[i]
	}
	for i := range a1p { //@ diag(`should use copy`)
		a2p[i] = a1p[i]
	}
	for i := range a1p { //@ diag(`should copy arrays using assignment`)
		a3p[i] = a1p[i]
	}
}
