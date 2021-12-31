
pub struct Asm {
    input :String,
    asm: Vec<String>,
}

impl Asm {
    pub fn new(input: String) -> Self{
        let asm  = input
            .split(|c| c == '\n' || c == ';')
            .map(|s| s.to_string())
            .collect();
        return Self {
            input: input,
            asm: asm
        }
    }
    pub fn list(&self) {
        for l in self.asm.iter() {
            println!("{}", l);
        }
    }
}