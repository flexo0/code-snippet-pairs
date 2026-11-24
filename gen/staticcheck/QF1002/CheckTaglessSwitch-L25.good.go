package p

func fn1() {
	switch a { //@ diag(`could use tagged switch on a`)
	case x == y, x != y:
	}
}
