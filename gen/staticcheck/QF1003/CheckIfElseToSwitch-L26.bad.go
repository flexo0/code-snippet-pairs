package p

func fn() {
	if z[0] == 1 || z[0] == 2 { //@ diag(`could use tagged switch on z[0]`)
	} else if z[0] == 3 {
	}
}
