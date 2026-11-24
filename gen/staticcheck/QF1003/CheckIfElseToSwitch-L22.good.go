package p

func fn() {
	switch a {
	case x == y: //@ diag(`could use tagged switch on a`)
	case x != y:
	}
}
