pub struct Asm {
    input: String,
    asm: Vec<String>,
    cur: usize,
    pub mapped_mem: *mut u8,
    mapped_mem_size: usize,
    cur_mapped_mem_offset: usize,
}
impl Asm {
    pub fn new(input: String) -> Self {
        let asm = input
            .split(|c| c == '\n' || c == ';')
            .map(|s| s.to_string())
            .collect();
        return Self {
            input: input,
            asm: asm,
            cur: 0,
            mapped_mem: 0 as *mut u8,
            mapped_mem_size: 0,
            cur_mapped_mem_offset: 0,
        };
    }
    pub fn list(&self) {
        for l in self.asm.iter() {
            println!("{}", l);
        }
    }
    pub fn set_mapped_mem(&mut self, mem: *mut u8, size: usize) {
        self.mapped_mem = mem;
        self.mapped_mem_size = size;
    }
    pub fn assemble(&mut self) {
        while self.cur != self.asm.len() {
            let byte_code = compile(self.asm[self.cur].clone());
            self.write_mem(byte_code);
            self.cur += 1;
        }
    }
    fn write_mem(&mut self, byte_code: Vec<u8>) {
        let size = byte_code.len();
        let cur_pos = self.mapped_mem as usize + self.cur_mapped_mem_offset;
        for i in 0..size {
            unsafe {
                *((cur_pos + i) as *mut u8) = byte_code[i];
            }
        }
        self.cur_mapped_mem_offset += size;
        return;
    }
    pub fn run(&self) {
        let f = unsafe { std::mem::transmute::<*mut (), fn()>(self.mapped_mem as *mut ()) };
        f();
    }
}

fn compile(input: String) -> Vec<u8> {
    if input == "nop".to_string() {
        return vec![0x90];
    } else if input == "ret".to_string() {
        return vec![0xc3];
    } else {
        panic!("{} : not impl yet...", input);
    }
}
