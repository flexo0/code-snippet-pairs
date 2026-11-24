impl MoveStruct {
    pub fn ref_func(&self) -> Option<Vec<u32>> {
        if self.opt.is_none() {
            //~^ question_mark
            return None;
        }
    }
}
