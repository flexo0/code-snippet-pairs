package p

func fn5() {
	for i := 0; i < len(src); i++ { //@ diag(`should use copy`)
		dst[i] = src[i]
	}
}
