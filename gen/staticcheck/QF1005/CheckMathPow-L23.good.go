package p

func fn() {
	_ = x * 2 * x * 2     //@ diag(`could expand call to math.Pow`)
	_ = (x + 2) * (x + 2) //@ diag(`could expand call to math.Pow`)
}
