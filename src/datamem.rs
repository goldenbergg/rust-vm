pub struct DataMem {
    main_mem: Vec<i32>,
    sub_mem: Vec<Vec<i32>>,
}

impl DataMem {
    pub fn new(size: usize) -> DataMem {
        let mut data_mem = DataMem {
            main_mem: Vec::with_capacity(size),
            sub_mem: Vec::new(),
        };

        data_mem.main_mem.resize(size, 0);
        data_mem
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
        let pos = self.sub_mem.len() - 1;
        self.sub_mem[pos].resize(size, 0);
    }

    pub fn pop_sub_mem(&mut self) {
        self.sub_mem.pop();
    }
}