package p

func fn1() {
	switch { //@ diag(`could use tagged switch on a`)
	case a == (x == y) || a == (x != y):
	}
}
