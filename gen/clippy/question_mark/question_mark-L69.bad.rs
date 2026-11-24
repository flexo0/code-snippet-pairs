impl CopyStruct {
    pub fn func(&self) -> Option<u32> {
        let _ = if self.opt.is_none() {
        //~^ question_mark
            return None;
        } else {
            self.opt
        };
    }
}
