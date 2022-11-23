pub struct DataMem {
    main_mem: Vec<i32>,
    sub_mem: Vec<Vec<i32>>,
}

impl DataMem {
    pub fn new(size: usize) -> DataMem {
        DataMem {
            main_mem: Vec::with_capacity(size),
            sub_mem: Vec::new(),
        }
    }

    pub fn get_data(&self, pos: usize) -> i32 {
        if pos as i32 > (self.main_mem.capacity() as i32 - 1) {
            self.sub_mem.last().unwrap()[pos - self.main_mem.capacity()]
        } else {
            self.main_mem[pos]
        }
    }

    pub fn set_data(&mut self, pos: usize, data: i32) {
        if pos as i32 > (self.main_mem.capacity() as i32 - 1) {
            self.sub_mem.last_mut().unwrap().insert(pos - 
                self.main_mem.capacity(), data);
        } else {
            self.main_mem.insert(pos, data);
        }
    }

    pub fn push_sub_mem(&mut self, size: usize) {
        self.sub_mem.push(Vec::with_capacity(size));
    }

    pub fn pop_sub_mem(&mut self) {
        self.sub_mem.pop();
    }
}