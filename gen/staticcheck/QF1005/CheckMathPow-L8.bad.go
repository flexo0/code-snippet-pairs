package p

func fn() {
	_ = math.Pow(x, 0) //@ diag(`could expand call to math.Pow`)
	_ = math.Pow(x, 1) //@ diag(`could expand call to math.Pow`)
	_ = math.Pow(x, 2) //@ diag(`could expand call to math.Pow`)
	_ = math.Pow(x, 3) //@ diag(`could expand call to math.Pow`)
}
