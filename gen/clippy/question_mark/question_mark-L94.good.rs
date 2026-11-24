impl MoveStruct {
    pub fn ref_func(&self) -> Option<Vec<u32>> {
        self.opt.as_ref()?;
    }
}
