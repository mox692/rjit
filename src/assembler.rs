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
impl Bit {
    fn to_bit(i: u64) -> Self {
        if i >> 7 == 0 {
            return Bit::Byte;
        } else if i >> 15 == 0 {
            return Bit::Word;
        } else if i >> 31 == 0 {
            return Bit::Double;
        } else if i >> 63 == 0 {
            return Bit::Quad;
        } else {
            panic!("fail to Bit, input: {}", i)
        }
    }
}

// Rm  -> register memory
// R   -> regisger
// Imm -> immediate
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RMImmType {
    // op1_op2 (ref: http://ref.x86asm.net/coder64.html#x0F6A)
    Imm_R(Bit, Bit), // first: Imm substantial bit, second: register bit.
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

    Rax,
    Rcx,
    Rdx,
    Rbx,
}
impl Register {
    fn from_operand(operand: Operand) -> Self {
        match operand {
            | Operand::Reg(r) => r,
            | _ => panic!("operand {:?} is not register.", operand),
        }
    }
    fn index(&self) -> u8 {
        match self {
            | Register::Eax => 0,
            | Register::Ecx => 1,
            | Register::Edx => 2,
            | Register::Ebx => 3,

            | Register::Rax => 0,
            | Register::Rcx => 1,
            | Register::Rdx => 2,
            | Register::Rbx => 3,
        }
    }
    fn get_bit(&self) -> Bit {
        match self {
            | Register::Eax => Bit::Double,
            | Register::Ecx => Bit::Double,
            | Register::Edx => Bit::Double,
            | Register::Ebx => Bit::Double,

            | Register::Rax => Bit::Quad,
            | Register::Rcx => Bit::Quad,
            | Register::Rdx => Bit::Quad,
            | Register::Rbx => Bit::Quad,
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
    Int,
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
    fn imm_from_operand_u64(operand: Operand) -> u64 {
        match operand {
            | Operand::Imm(i) => i as u64,
            | _ => panic!("operand {:?} is not imm.", operand),
        }
    }
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
    fn imm_from_operand_u8(operand: Operand) -> u8 {
        match operand {
            | Operand::Imm(i) => i as u8,
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
        | InstructionType::Int => parse_int(instruction),
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
    let mut code: Vec<u8> = vec![];
    match instruction.rmimm_type {
        | RMImmType::Imm_R(imm_bit, reg_bit) => {
            let reg = Register::from_operand(instruction.second_op);
            let mut reg_bit_cp = reg_bit.clone();

            // emit Opecode
            let mut op_code = match reg_bit {
                | Bit::Double => vec![0xb8 + reg.index()],
                | Bit::Quad => match imm_bit {
                    // レジスタは64bitで指定してあるが、即値が32bitで収まる値だった場合、
                    // 即値を4byteでエンコードする.
                    | Bit::Double | Bit::Byte => {
                        reg_bit_cp = Bit::Double;
                        vec![0x48, 0xc7, 0xc0 + reg.index()]
                    }
                    | Bit::Quad => vec![0x48, 0xb8 + reg.index()],
                    | _ => panic!("Not impement."),
                },
                | _ => panic!("Unimplement."),
            };
            code.append(&mut op_code);

            // emit Immediate
            let mut imm = match reg_bit_cp {
                | Bit::Byte => Operand::imm_from_operand_u16(instruction.first_op)
                    .to_le_bytes()
                    .to_vec(),
                | Bit::Double => Operand::imm_from_operand_u32(instruction.first_op)
                    .to_le_bytes()
                    .to_vec(),
                | Bit::Quad => Operand::imm_from_operand_u64(instruction.first_op)
                    .to_le_bytes()
                    .to_vec(),
                | _ => panic!("not implemented."),
            };
            code.append(&mut imm)
        }

        | RMImmType::R_Rm(reg_bit) => {
            let mut op_code: Vec<u8> = match reg_bit {
                | Bit::Double => vec![0x89],
                | Bit::Quad => vec![0x48, 0x89],
                | _ => panic!("Not implement."),
            };
            code.append(&mut op_code);

            let first_reg = Register::from_operand(instruction.first_op);
            let second_reg = Register::from_operand(instruction.second_op);
            let m: u8 = 0b11;
            let reg = second_reg.index(); // 演算結果を格納する方のreg
            let rm = first_reg.index(); // 格納する方
            let modrm = reg | rm << 3 | m << 6;

            code.append(&mut vec![modrm]);
        }
        | _ => panic!("Not implemet."),
    }

    return code;
}
fn parse_int(instruction: Instruction) -> Vec<u8> {
    let immediate = Operand::imm_from_operand_u8(instruction.first_op);
    return vec![0xcd, immediate]
}

fn parse_instruction(tok: Vec<String>) -> Instruction {
    let tok_len = tok.len();

    let typ: InstructionType = match tok[0].as_str() {
        | "nop" => InstructionType::Nop,
        | "ret" => InstructionType::Ret,
        | "mov" => InstructionType::Mov,
        | "add" => InstructionType::Add,
        | "sub" => InstructionType::Sub,
        | "int" => InstructionType::Int,
        | _ => panic!("Unknown Instruction, {:?}", tok[0].as_str()),
    };

    // instruction which does not take an operand.
    if tok_len == 1 {
        return Instruction {
            typ: typ,
            rmimm_type: RMImmType::Other,
            first_op: Operand::None,
            second_op: Operand::None,
        };
    }

    let mut operands: Vec<Operand> = vec![];
    for s in tok.iter().cloned().skip(1) {
        let operand = parse_operand(s);
        operands.push(operand);
    }

    // instruction which takes one operand.
    if tok_len == 2 {
        return Instruction {
            typ: typ,
            rmimm_type: RMImmType::Other,
            first_op: operands[0].clone(),
            second_op: Operand::None,
        };
    }

    // instruction which takes two operands.
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
            let imm_bit = Bit::to_bit(Operand::imm_from_operand_u64(operands[0].clone()));
            let register_bit = Register::from_operand(operands[1].clone()).get_bit();
            RMImmType::Imm_R(imm_bit, register_bit)
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
                | "ecx" => Operand::Reg(Register::Edx),
                | "edx" => Operand::Reg(Register::Edx),
                | "ebx" => Operand::Reg(Register::Edx),
                | "rax" => Operand::Reg(Register::Rax),
                | "rcx" => Operand::Reg(Register::Rcx),
                | "rdx" => Operand::Reg(Register::Rdx),
                | "rbx" => Operand::Reg(Register::Rbx),
                | _ => panic!("Unknown Register Name: {:?}", reg_name),
            };
        }
        | '$' => {
            let mut num_str = op_chars.collect::<String>();
            let num: u64;
            if num_str.contains("0x") {
                num_str.remove_matches("0x");
                num = u64::from_str_radix(num_str.as_str(), 16).unwrap();
            } else {
                num = u64::from_str_radix(num_str.as_str(), 10).unwrap_or_else(|e| {
                    panic!(
                        "num_str: {}, invalid. Using Unsupported prefix? err: {}",
                        num_str, e
                    )
                })
            }
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
            AssembleTestCase {
                input: "mov $0x11223344, %rax".to_string(),
                expect: vec![0x48, 0xc7, 0xc0, 0x44, 0x33, 0x22, 0x11],
            },
            AssembleTestCase {
                input: "mov %rax, %rax".to_string(),
                expect: vec![0x48, 0x89, 0xc0],
            },
            AssembleTestCase {
                input: "mov %eax, %eax".to_string(),
                expect: vec![0x89, 0xc0],
            },
            AssembleTestCase {
                input: "mov %eax, %edx".to_string(),
                expect: vec![0x89, 0xc2],
            },
            AssembleTestCase {
                input: "mov   %rdx, %rax".to_string(),
                expect: vec![0x48, 0x89, 0xd0],
            },
            AssembleTestCase {
                input: "int $0x80".to_string(),
                expect: vec![0xcd, 0x80],
            },
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
            ParseOperandTestCase {
                input: "$0x1234".to_string(),
                expect: Operand::Imm(4660),
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
        let test_case = vec![ParseMovTestCase {
            input: Instruction {
                typ: InstructionType::Mov,
                rmimm_type: RMImmType::Imm_R(Bit::Double, Bit::Double),
                first_op: Operand::Imm(0x11223344),
                second_op: Operand::Reg(Register::Eax),
            },
            expect: vec![0xb8, 0x44, 0x33, 0x22, 0x11],
        }];
        for t in test_case {
            let res = parse_mov(t.input);
            assert_eq!(res, t.expect);
        }
    }
}
