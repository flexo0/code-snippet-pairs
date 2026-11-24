package p

func fn() {
	for {
		if x && (a == b) { //@ diag(`could lift into loop condition`)
			break
		}
	}
}
