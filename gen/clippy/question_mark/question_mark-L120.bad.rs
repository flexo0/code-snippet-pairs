impl MoveStruct {
    pub fn if_let_ref_func(self) -> Option<Vec<u32>> {
        let v: &Vec<_> = if let Some(ref v) = self.opt {
            //~^ question_mark
            v
        } else {
            return None;
        };
    }
}
