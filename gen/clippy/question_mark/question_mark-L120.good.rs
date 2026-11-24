impl MoveStruct {
    pub fn if_let_ref_func(self) -> Option<Vec<u32>> {
        let v: &Vec<_> = self.opt.as_ref()?;
    }
}
