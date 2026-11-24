package p

func fn1() {
	switch z[""][0] { //@ diag(`could use tagged switch on z[""][0]`)
	case 1, 2:
	}
}
