package p

func fn4() {
	copy(a1p[:], b)
	copy(b, a1p[:])
	copy(a2p[:], a1p[:])
	*a3p = *a1p
}
