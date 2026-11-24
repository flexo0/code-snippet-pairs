package p

func fn() {
	switch z[0] {
	case 1, 2: //@ diag(`could use tagged switch on z[0]`)
	case 3:
	}
}
