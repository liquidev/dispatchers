#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Opcode {
    Int,

    Let,
    Var,

    LessEq,
    Add,
    Multiply,

    JumpIfNot,
    Jump,
    Halt,
}

struct Frame<'c> {
    stack: [u32; 256],
    sp: usize,
    bytecode: &'c [u8],
    pc: u16,
}

impl<'c> Frame<'c> {
    fn read_u8(&mut self) -> u8 {
        debug_assert!((self.pc as usize) < self.bytecode.len());
        let x = *unsafe { self.bytecode.get_unchecked(self.pc as usize) };
        self.pc += 1;
        x
    }

    fn read_u16(&mut self) -> u16 {
        debug_assert!((self.pc as usize + 1) < self.bytecode.len());
        let x = unsafe {
            *std::mem::transmute::<_, &u16>(self.bytecode.get_unchecked(self.pc as usize))
        };
        self.pc += 2;
        u16::from_le(x)
    }

    fn read_u32(&mut self) -> u32 {
        debug_assert!((self.pc as usize + 3) < self.bytecode.len());
        let x = unsafe {
            *std::mem::transmute::<_, &u32>(self.bytecode.get_unchecked(self.pc as usize))
        };
        self.pc += 4;
        u32::from_le(x)
    }

    fn var(&self, i: u8) -> u32 {
        debug_assert!((i as usize) < self.stack.len());
        unsafe { *self.stack.get_unchecked(i as usize) }
    }

    fn set_var(&mut self, i: u8, val: u32) {
        debug_assert!((i as usize) < self.stack.len());
        unsafe {
            *self.stack.get_unchecked_mut(i as usize) = val;
        }
    }

    fn push(&mut self, x: u32) {
        unsafe {
            *self.stack.get_unchecked_mut(self.sp) = x;
        }
        self.sp += 1;
    }

    fn pop(&mut self) -> u32 {
        let x = *unsafe { self.stack.get_unchecked(self.sp - 1) };
        self.sp -= 1;
        x
    }
}

impl<'c> Frame<'c> {
    #[inline(never)]
    fn eval(&mut self) {
        loop {
            let opcode = unsafe { std::mem::transmute::<_, Opcode>(self.read_u8()) };
            match opcode {
                Opcode::Int => {
                    let i = self.read_u32();
                    self.push(i);
                }
                Opcode::Let => {
                    let i = self.read_u8();
                    let val = self.pop();
                    self.set_var(i, val);
                }
                Opcode::Var => {
                    let i = self.read_u8();
                    let val = self.var(i);
                    self.push(val);
                }
                Opcode::LessEq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a <= b) as u32);
                }
                Opcode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                }
                Opcode::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }
                Opcode::JumpIfNot => {
                    let offset = self.read_u16();
                    let condition = self.pop();
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
        // println!("{:?}", &self.stack[0..self.sp]);
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

pub fn code() -> Vec<u8> {
    let mut w = Writer::default();

    // i = 1
    w.write_opcode(Opcode::Int);
    w.write_u32(1);

    // x = 1
    w.write_opcode(Opcode::Int);
    w.write_u32(1);

    let loop_start = w.pc();

    // i <= n
    {
        w.write_opcode(Opcode::Var);
        w.write_u8(VAR_I);

        w.write_opcode(Opcode::Var);
        w.write_u8(VAR_N);

        w.write_opcode(Opcode::LessEq);
    }

    w.write_opcode(Opcode::JumpIfNot);
    let loop_jump_hole = w.write_u16(0);

    // x = x * i
    {
        w.write_opcode(Opcode::Var);
        w.write_u8(VAR_X);

        w.write_opcode(Opcode::Var);
        w.write_u8(VAR_I);

        w.write_opcode(Opcode::Multiply);

        w.write_opcode(Opcode::Let);
        w.write_u8(VAR_X);
    }

    // i = i + 1
    {
        w.write_opcode(Opcode::Var);
        w.write_u8(VAR_I);

        w.write_opcode(Opcode::Int);
        w.write_u32(1);

        w.write_opcode(Opcode::Add);

        w.write_opcode(Opcode::Let);
        w.write_u8(VAR_I);
    }

    w.write_opcode(Opcode::Jump);
    w.write_u16(loop_start);

    let loop_end = w.pc();
    w.write_opcode(Opcode::Var);
    w.write_u8(VAR_X);

    w.patch_u16(loop_jump_hole, loop_end);
    w.write_opcode(Opcode::Halt);

    w.bytecode
}

pub fn run(code: &[u8]) -> u32 {
    let mut frame = Frame {
        stack: [0; 256],
        sp: 0,
        bytecode: code,
        pc: 0,
    };
    frame.push(10);
    frame.eval();
    frame.pop()
}
