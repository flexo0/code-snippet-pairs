package p

func fn() {
	_ = math.Pow(x*2, 2) //@ diag(`could expand call to math.Pow`)
	_ = math.Pow(x+2, 2) //@ diag(`could expand call to math.Pow`)
}
