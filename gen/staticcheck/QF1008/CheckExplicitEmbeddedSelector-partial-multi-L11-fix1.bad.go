package p

func fnPartialMulti() {
	_ = partialMulti.F3.BasicOuter.F1            //@ diag(`could remove embedded field "BasicOuter" from selector`)
	_ = partialMulti.F3.BasicOuter.BasicInner.F1 //@ diag(`could remove embedded field "BasicOuter" from selector`), diag(`could remove embedded field "BasicInner" from selector`)
	_ = partialMulti.F3.F1                       // minimal form
}
