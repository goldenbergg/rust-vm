pub struct StrBuf {
    def_buf: Vec<String>,
}

impl StrBuf {
    pub fn new(size: usize) -> StrBuf {
        StrBuf {
            def_buf: Vec::with_capacity(size),
        }
    }

    pub fn get_size(&self) -> usize{
        self.def_buf.capacity()
    }

    pub fn get_str(&self, pos: usize) -> String {
        self.def_buf[pos].clone()
    }

    pub fn insert_at(&mut self, pos: usize, str: String) {
        self.def_buf.insert(pos, str);
    }
}