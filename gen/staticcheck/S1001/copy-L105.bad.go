package p

func fn4() {
	for i := range a1 { //@ diag(`should use copy`)
		a2p[i] = a1[i]
	}
	for i := range a1 { //@ diag(`should copy arrays using assignment`)
		a3p[i] = a1[i]
	}
	for i := range a1p { //@ diag(`should use copy`)
		a2[i] = a1p[i]
	}
	for i := range a1p { //@ diag(`should copy arrays using assignment`)
		a3[i] = a1p[i]
	}
}
