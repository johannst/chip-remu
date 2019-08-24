pub struct Memory {
    mem: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; 4096] }
    }

    pub fn load(&mut self, addr: u16, data: &[u8]) {
        let addr = addr as usize;
        assert!(addr + data.len() < self.mem.len());
        self.mem[addr..addr + data.len()].copy_from_slice(data);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        assert!(addr < (self.mem.len() as u16));
        self.mem[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, data : u8) {
        assert!(addr < (self.mem.len() as u16));
        self.mem[addr as usize] = data;
    }

    pub fn dump_range(&self, addr: usize, size: usize) {
        if addr > self.mem.len() {
            return;
        }

        let mut addr = addr;
        let mut end_addr = addr + size;
        if end_addr > self.mem.len() {
            end_addr = self.mem.len();
        }
        let end_addr = end_addr;

        println!("---- MEM dump ----");
        for vals in self.mem[addr..end_addr].chunks(16) {
            print!("0x{:04x}: ", addr);
            for val in vals {
                print!("0x{:02x} ", val);
            }
            println!("");
            addr += vals.len();
        }
        println!("------------------");
    }

    pub fn dump(&self) {
        self.dump_range(0x0, self.mem.len());
    }
}
