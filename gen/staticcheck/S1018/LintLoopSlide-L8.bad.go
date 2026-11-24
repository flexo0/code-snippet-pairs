package p

func fn() {
	for i := 0; i < n; i++ { //@ diag(`should use copy() instead of loop for sliding slice elements`)
		bs[i] = bs[offset+i]
	}
}
