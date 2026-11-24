package p

func fn4() {
	copy(a2p[:], a1[:])
	*a3p = a1
	copy(a2[:], a1p[:])
	a3 = *a1p
}
