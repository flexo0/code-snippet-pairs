package p

func fn() {
	if _, ok := m2["k"]; ok { //@ diag(`unnecessary guard around map access`)
		m2["k"] += 4
	} else {
		m2["k"] = 4
	}
}
