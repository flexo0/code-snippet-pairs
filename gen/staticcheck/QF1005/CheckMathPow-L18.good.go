package p

func fn() {
	_ = float64(a * a)    //@ diag(`could expand call to math.Pow`)
	_ = b * b             //@ diag(`could expand call to math.Pow`)
	_ = c * c             //@ diag(`could expand call to math.Pow`)
	_ = a * 1.0 * a * 1.0 //@ diag(`could expand call to math.Pow`)
}
