use libc::{mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::ptr::{self};

pub struct Assembler {
    input: String,
    asm: Vec<String>,
    cur: usize,
    pub mapped_mem: *mut u8,
    mapped_mem_size: usize,
    cur_mapped_mem_offset: usize,
}
impl Assembler {
    pub fn new(input: String) -> Self {
        let asm = input
            .split(|c| c == '\n' || c == ';')
            .map(|s| s.to_string())
            .collect();

        let mmap = unsafe {
            // TODO: fixed 1000 bytes.
            mmap(
                ptr::null_mut(),
                1000,
                PROT_EXEC | PROT_READ | PROT_WRITE,
                MAP_ANONYMOUS | MAP_PRIVATE,
                -1,
                0,
            )
        };

        return Self {
            input: input,
            asm: asm,
            cur: 0,
            mapped_mem: mmap as *mut u8,
            mapped_mem_size: 1000,
            cur_mapped_mem_offset: 0,
        };
    }
    pub fn list(&self) {
        for l in self.asm.iter() {
            println!("{}", l);
        }
    }
    pub fn assemble(&mut self) {
        while self.cur != self.asm.len() {
            let byte_code = assemble(self.asm[self.cur].clone());
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
        let f =
            unsafe { std::mem::transmute::<*mut (), fn()>(self.mapped_mem as *mut ()) };
        f();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Bit {
    Byte,
    Word,
    Double,
    Quad,
}
// Rm  -> register memory
// R   -> regisger
// Imm -> immediate
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RMImmType {
    // op1_op2 (ref: http://ref.x86asm.net/coder64.html#x0F6A)
    Imm_R(Bit),
    Rm_R(Bit),
    R_Rm(Bit),
    Other,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Register {
    Eax,
    Ecx,
    Edx,
    Ebx,
}
impl Register {
    fn from_operand(operand: Operand) -> Self {
        match operand {
            | Operand::Reg(r) => r,
            | _ => panic!("operand {:?} is not register.", operand),
        }
    }
    fn index(&self) -> usize {
        match self {
            | Register::Eax => 0,
            | Register::Ecx => 1,
            | Register::Edx => 2,
            | Register::Ebx => 3,
        }
    }
    fn get_bit(&self) -> Bit {
        match self {
            | Eax => Bit::Double,
            | Ecx => Bit::Double,
            | Edx => Bit::Double,
            | Ebx => Bit::Double,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    Nop,
    Ret,
    Mov,
    Add,
    Sub,
    Unknown,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Reg(Register),
    Mem(*mut u8),
    Imm(u64),
    None,
}
impl Operand {
    fn imm_from_operand_u32(operand: Operand) -> u32 {
        match operand {
            | Operand::Imm(i) => i as u32,
            | _ => panic!("operand {:?} is not imm.", operand),
        }
    }
    fn imm_from_operand_u16(operand: Operand) -> u16 {
        match operand {
            | Operand::Imm(i) => i as u16,
            | _ => panic!("operand {:?} is not imm.", operand),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    typ: InstructionType,
    rmimm_type: RMImmType,
    first_op: Operand,
    second_op: Operand,
}
impl Instruction {}
impl Default for Instruction {
    fn default() -> Self {
        return Self {
            typ: InstructionType::Unknown,
            rmimm_type: RMImmType::Other,
            first_op: Operand::None,
            second_op: Operand::None,
        };
    }
}

// asmの1行がここに入るイメージ
pub fn assemble(input: String) -> Vec<u8> {
    let tok = tokenize(input.clone());
    let instruction = parse_instruction(tok);

    match instruction.typ {
        | InstructionType::Nop => parse_nop(instruction),
        | InstructionType::Ret => parse_ret(instruction),
        | InstructionType::Add => parse_add(instruction),
        | InstructionType::Mov => parse_mov(instruction),
        | _ => panic!("unimplement."),
    }
}

fn parse_nop(input: Instruction) -> Vec<u8> {
    return vec![0x90];
}
fn parse_ret(input: Instruction) -> Vec<u8> {
    return vec![0xc3];
}
fn parse_add(input: Instruction) -> Vec<u8> {
    return vec![];
}
fn parse_mov(instruction: Instruction) -> Vec<u8> {
    // AssembleTestCase{
    //     input: "mov	$0x11223344, %eax".to_string(),
    //     expect: vec![0xb8, 0x44, 0x33, 0x22, 0x11],
    // },
    // match input.first_op {
    // }

    // ↑だと、
    let mut code: Vec<u8> = vec![];
    match instruction.rmimm_type {
        | RMImmType::Imm_R(bit) => {
            let reg = Register::from_operand(instruction.second_op);
            // TODO: ここのopcodeの生成ロジックは64bit対応する時にもう少し複雑になるはず.
            // emit Opecode
            let op_code = 0xb8 + reg.index() as u8;
            code.push(op_code);

            // emit Immediate
            let mut imm = match bit {
                | Bit::Byte => Operand::imm_from_operand_u16(instruction.first_op)
                    .to_le_bytes()
                    .to_vec(),
                | Bit::Double => Operand::imm_from_operand_u32(instruction.first_op)
                    .to_le_bytes()
                    .to_vec(),
                | _ => panic!("not implemented."),
            };
            code.append(&mut imm)
        }
        | _ => panic!("Not implemet."),
    }

    return code;
}

fn parse_instruction(tok: Vec<String>) -> Instruction {
    let typ: InstructionType = match tok[0].as_str() {
        | "nop" => InstructionType::Nop,
        | "ret" => InstructionType::Ret,
        | "mov" => InstructionType::Mov,
        | "add" => InstructionType::Add,
        | "sub" => InstructionType::Sub,
        | _ => panic!("Unknown Instruction, {:?}", tok[0].as_str()),
    };

    // instruction which does not take an operand.
    if tok.len() == 1 {
        return Instruction {
            typ: typ,
            rmimm_type: RMImmType::Other,
            first_op: Operand::None,
            second_op: Operand::None,
        };
    }

    // instruction which does take an operand.
    let mut operands: Vec<Operand> = vec![];
    for s in tok.iter().cloned().skip(1) {
        let operand = parse_operand(s);
        operands.push(operand);
    }

    let rmimm_type = get_rmimm_type(operands.clone());

    return Instruction {
        typ: typ,
        rmimm_type: rmimm_type,
        first_op: operands[0].clone(),
        second_op: operands[1].clone(),
    };
}

fn get_rmimm_type(operands: Vec<Operand>) -> RMImmType {
    // TODO: operandを2つとる系の命令だけ、さしあたり考える
    // MEMO: ここでのoperands[0] は mov %rax, (%rdi) とした時の%raxをさす。(参考サイトのoperand1とは違う!!!)
    match operands[0] {
        | Operand::Mem(_) => {
            RMImmType::Rm_R(Register::from_operand(operands[1].clone()).get_bit())
        }
        | Operand::Reg(_) => {
            RMImmType::R_Rm(Register::from_operand(operands[0].clone()).get_bit())
        }
        | Operand::Imm(_) => {
            RMImmType::Imm_R(Register::from_operand(operands[1].clone()).get_bit())
        }
        | _ => panic!("Unkown Operand Type: {:?}", operands[0]),
    }
}

fn parse_operand(input: String) -> Operand {
    let mut op_chars = input.chars();

    match op_chars.nth(0).unwrap() {
        | '%' => {
            let mut reg_name = String::new();
            for c in op_chars {
                if !c.is_alphabetic() {
                    continue;
                }
                reg_name.push(c);
            }
            return match reg_name.as_str() {
                | "eax" => Operand::Reg(Register::Eax),
                | "edx" => Operand::Reg(Register::Edx),
                | _ => panic!("Unknown Register Name: {:?}", reg_name),
            };
        }
        | '$' => {
            let num_str = op_chars.collect::<String>();
            let num: u64 = num_str.parse().unwrap();
            return Operand::Imm(num);
        }
        | _ => panic!("Unexpected Operand: {:?}", op_chars),
    };
}

fn tokenize(input: String) -> Vec<String> {
    let mut tok: Vec<String> = input
        .split([' ', ','].as_ref())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    tok.retain(|s| s.clone() != "".to_string());

    if tok.len() == 0 || tok.len() > 3 {
        panic!("invalid!!, tok: {:?}", tok);
    }
    return tok;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AssembleTestCase {
        input: String,
        expect: Vec<u8>,
    }
    #[test]
    fn test_assemble() {
        let test_case = vec![
            AssembleTestCase {
                input: "nop".to_string(),
                expect: vec![0x90],
            },
            AssembleTestCase {
                input: "ret".to_string(),
                expect: vec![0xc3],
            },
            // // 32bit命令, 即値
            // AssembleTestCase{
            //     input: "mov	$0x11223344, %eax".to_string(),
            //     expect: vec![0xb8, 0x44, 0x33, 0x22, 0x11],
            // },
        ];
        for t in test_case {
            assert_eq!(assemble(t.input), t.expect)
        }
    }

    struct ParseInstructionTestCase {
        input: Vec<String>,
        expect: Instruction,
    }
    #[test]
    fn test_parse_instruction() {
        let test_case = vec![
            ParseInstructionTestCase {
                input: vec!["nop".to_string()],
                expect: Instruction {
                    typ: InstructionType::Nop,
                    rmimm_type: RMImmType::Other,
                    first_op: Operand::None,
                    second_op: Operand::None,
                },
            },
            ParseInstructionTestCase {
                input: vec!["add".to_string(), "%edx".to_string(), "%eax".to_string()],
                expect: Instruction {
                    typ: InstructionType::Add,
                    rmimm_type: RMImmType::R_Rm(Bit::Double),
                    first_op: Operand::Reg(Register::Edx),
                    second_op: Operand::Reg(Register::Eax),
                },
            },
        ];
        for t in test_case {
            let res = parse_instruction(t.input);
            assert_eq!(res, t.expect)
        }
    }

    struct TokenizeTestCase {
        input: String,
        expect: Vec<String>,
    }
    #[test]
    fn test_tokenize() {
        let test_case = vec![
            TokenizeTestCase {
                input: "nop".to_string(),
                expect: vec!["nop".to_string()],
            },
            TokenizeTestCase {
                input: "mov %edi, %eax".to_string(),
                expect: vec!["mov".to_string(), "%edi".to_string(), "%eax".to_string()],
            },
            TokenizeTestCase {
                input: "mov  %edi,          %eax".to_string(),
                expect: vec!["mov".to_string(), "%edi".to_string(), "%eax".to_string()],
            },
        ];
        for t in test_case {
            let res = tokenize(t.input);
            assert_eq!(res, t.expect);
        }
    }

    struct ParseOperandTestCase {
        input: String,
        expect: Operand,
    }
    #[test]
    fn test_parse_operand() {
        let test_case = vec![
            ParseOperandTestCase {
                input: "%eax".to_string(),
                expect: Operand::Reg(Register::Eax),
            },
            ParseOperandTestCase {
                input: "$1234".to_string(),
                expect: Operand::Imm(1234),
            },
        ];
        for t in test_case {
            let res = parse_operand(t.input);
            assert_eq!(res, t.expect)
        }
    }

    struct ParseMovTestCase {
        input: Instruction,
        expect: Vec<u8>,
    }
    #[test]
    fn test_parse_mov() {
        let test_case = vec![
            ParseMovTestCase {
                input: Instruction {
                    typ: InstructionType::Mov,
                    rmimm_type: RMImmType::Imm_R(Bit::Double),
                    first_op: Operand::Imm(0x11223344),
                    second_op: Operand::Reg(Register::Eax),
                },
                expect: vec![0xb8, 0x44, 0x33, 0x22, 0x11],
            },
            // ParseMovTestCase {
            //     input: Instruction {
            //         typ: InstructionType::Mov,
            //         rmimm_type: RMImmType::Imm_R,
            //         first_op: Operand::Imm(0x55),
            //         second_op: Operand::Reg(Register::Ecx),
            //     },
            //     expect: vec![0xb9, 0x55, 0x00, 0x00, 0x00],
            // },
        ];

        for t in test_case {
            let res = parse_mov(t.input);
            assert_eq!(res, t.expect);
        }
    }
}
