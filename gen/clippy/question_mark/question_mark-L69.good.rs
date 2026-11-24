impl CopyStruct {
    pub fn func(&self) -> Option<u32> {
        let _ = Some(self.opt?);
    }
}
