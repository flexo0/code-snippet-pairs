package p

func fn() {
	if a == (x == y) { //@ diag(`could use tagged switch on a`)
	} else if a == (x != y) {
	}
}
