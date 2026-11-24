package p

func fnPartialMulti() {
	_ = partialMulti.F3.F1            //@ diag(`could remove embedded field "BasicOuter" from selector`)
	_ = partialMulti.F3.BasicInner.F1 //@ diag(`could remove embedded field "BasicOuter" from selector`), diag(`could remove embedded field "BasicInner" from selector`)
	_ = partialMulti.F3.F1            // minimal form
}
