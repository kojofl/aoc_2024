pub struct HashMap<K> {
    inner: [Option<K>; u8::MAX as usize],
}

impl<K> HashMap<K> {
    pub fn new() -> Self {
        Self {
            inner: [const { None }; u8::MAX as usize],
        }
    }
}
