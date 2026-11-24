package p

func fn() {
	for {
		if done() { //@ diag(`could lift into loop condition`)
			break
		}
	}
}
