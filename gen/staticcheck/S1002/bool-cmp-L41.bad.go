package p

func fn() {
	if !!y == true { //@ diag(`simplified to y`)
	}
}
