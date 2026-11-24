package p

func fn(m map[int]int) {
	if _, ok := m[0]; ok { //@ diag(`unnecessary guard`)
		delete(m, 0)
	}
}
