const MEMORY_SIZE: usize = 65536;

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: Vec::with_capacity(MEMORY_SIZE),
        }
    }

    pub fn read_byte(&self, i: u16) -> u8 {
        self.data[i as usize]
    }

    pub fn read_word(&self, i: u16) -> u16 {
        (self.read_byte(i) as u16) | ((self.read_byte(i+1) as u16) << 8)
    }

    pub fn write_bytes<Data: AsRef<[u8]>>(&mut self, i: u16, data: Data) {
        let begin = i as usize;
        let end = begin + data.as_ref().len();
        self.data[begin..end].copy_from_slice(data.as_ref());
    }
}
