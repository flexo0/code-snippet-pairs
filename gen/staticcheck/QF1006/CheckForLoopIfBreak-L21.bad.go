package p

func fn() {
	for {
		if a > b || b > a { //@ diag(`could lift into loop condition`)
			break
		}
	}
}
