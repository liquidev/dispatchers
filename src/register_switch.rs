#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Opcode {
    Int,

    LessEq,
    Add,
    Multiply,

    JumpIfNot,
    Jump,
    Halt,
}

struct Frame<'c> {
    variables: [u32; 256],
    bytecode: &'c [u8],
    pc: u16,
}

impl<'c> Frame<'c> {
    fn read_u8(&mut self) -> u8 {
        let x = self.bytecode[self.pc as usize];
        self.pc += 1;
        x
    }

    fn read_u16(&mut self) -> u16 {
        let x = unsafe { *std::mem::transmute::<_, &u16>(&self.bytecode[self.pc as usize]) };
        self.pc += 2;
        u16::from_le(x)
    }

    fn read_u32(&mut self) -> u32 {
        let x = unsafe { *std::mem::transmute::<_, &u32>(&self.bytecode[self.pc as usize]) };
        self.pc += 4;
        u32::from_le(x)
    }
}

impl<'c> Frame<'c> {
    fn eval(&mut self) {
        loop {
            let opcode = unsafe { std::mem::transmute::<_, Opcode>(self.read_u8()) };
            match opcode {
                Opcode::Int => {
                    let target = self.read_u8();
                    let i = self.read_u32();
                    self.variables[target as usize] = i;
                }
                Opcode::LessEq => {
                    let a = self.variables[self.read_u8() as usize];
                    let b = self.variables[self.read_u8() as usize];
                    let target = self.read_u8();
                    self.variables[target as usize] = (a <= b) as u32;
                }
                Opcode::Add => {
                    let a = self.variables[self.read_u8() as usize];
                    let b = self.variables[self.read_u8() as usize];
                    let target = self.read_u8();
                    self.variables[target as usize] = a + b;
                }
                Opcode::Multiply => {
                    let a = self.variables[self.read_u8() as usize];
                    let b = self.variables[self.read_u8() as usize];
                    let target = self.read_u8();
                    self.variables[target as usize] = a * b;
                }
                Opcode::JumpIfNot => {
                    let offset = self.read_u16();
                    let source = self.read_u8();
                    let condition = self.variables[source as usize];
                    if condition == 0 {
                        self.pc = offset;
                    }
                }
                Opcode::Jump => {
                    let offset = self.read_u16();
                    self.pc = offset;
                }
                Opcode::Halt => break,
            }
            self.dump();
        }
    }

    fn dump(&self) {
        // println!("{:?}", &self.variables[0..8]);
    }
}

#[derive(Default)]
struct Writer {
    bytecode: Vec<u8>,
}

impl Writer {
    fn write_u8(&mut self, x: u8) -> usize {
        let i = self.bytecode.len();
        self.bytecode.push(x);
        i
    }

    fn pc(&self) -> u16 {
        self.bytecode.len() as u16
    }

    fn write_u16(&mut self, x: u16) -> usize {
        let i = self.bytecode.len();
        self.bytecode.extend_from_slice(&x.to_le_bytes());
        i
    }

    fn patch_u16(&mut self, at: usize, x: u16) {
        let s = x.to_le_bytes();
        self.bytecode[at] = s[0];
        self.bytecode[at + 1] = s[1];
    }

    fn write_u32(&mut self, x: u32) -> usize {
        let i = self.bytecode.len();
        self.bytecode.extend_from_slice(&x.to_le_bytes());
        i
    }

    fn write_opcode(&mut self, opcode: Opcode) {
        self.write_u8(opcode as u8);
    }
}

const VAR_N: u8 = 0;
const VAR_I: u8 = 1;
const VAR_X: u8 = 2;

const TEMP: u8 = 3;

pub fn code() -> Vec<u8> {
    let mut w = Writer::default();

    w.write_opcode(Opcode::Int);
    w.write_u8(VAR_I);
    w.write_u32(1);

    w.write_opcode(Opcode::Int);
    w.write_u8(VAR_X);
    w.write_u32(1);

    let loop_start = w.pc();
    w.write_opcode(Opcode::LessEq);
    w.write_u8(VAR_I);
    w.write_u8(VAR_N);
    w.write_u8(TEMP);

    w.write_opcode(Opcode::JumpIfNot);
    let loop_jump_hole = w.write_u16(0);
    w.write_u8(TEMP);

    w.write_opcode(Opcode::Multiply);
    w.write_u8(VAR_X);
    w.write_u8(VAR_I);
    w.write_u8(VAR_X);

    w.write_opcode(Opcode::Int);
    w.write_u8(TEMP);
    w.write_u32(1);

    w.write_opcode(Opcode::Add);
    w.write_u8(VAR_I);
    w.write_u8(TEMP);
    w.write_u8(VAR_I);

    w.write_opcode(Opcode::Jump);
    w.write_u16(loop_start);

    let loop_end = w.pc();
    w.patch_u16(loop_jump_hole, loop_end);
    w.write_opcode(Opcode::Halt);

    w.bytecode
}

pub fn run(code: &[u8]) -> u32 {
    let mut frame = Frame {
        variables: [0; 256],
        bytecode: code,
        pc: 0,
    };
    frame.variables[VAR_N as usize] = 10;
    frame.eval();
    frame.variables[VAR_X as usize]
}
