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
        debug_assert!((self.pc as usize) < self.bytecode.len());
        let x = *unsafe { self.bytecode.get_unchecked(self.pc as usize) };
        self.pc += 1;
        x
    }

    fn peek_u8(&self) -> u8 {
        debug_assert!((self.pc as usize) < self.bytecode.len());
        *unsafe { self.bytecode.get_unchecked(self.pc as usize) }
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
        debug_assert!((i as usize) < self.variables.len());
        unsafe { *self.variables.get_unchecked(i as usize) }
    }

    fn set_var(&mut self, i: u8, val: u32) {
        debug_assert!((i as usize) < self.variables.len());
        unsafe {
            *self.variables.get_unchecked_mut(i as usize) = val;
        }
    }
}

static DISPATCH_TABLE: [fn(&mut Frame); 6] = [
    exec_int,
    exec_less_eq,
    exec_add,
    exec_multiply,
    exec_jump_if_not,
    exec_jump,
];

impl<'c> Frame<'c> {
    fn step(&mut self) {
        // print!("{:4} | ", self.pc);
        let opcode = self.read_u8();
        // println!("{:?}", unsafe { std::mem::transmute::<_, Opcode>(opcode) });
        unsafe { DISPATCH_TABLE.get_unchecked(opcode as usize)(self) };
    }

    fn eval(&mut self) {
        while self.peek_u8() != Opcode::Halt as u8 {
            self.step();
        }
    }

    fn dump(&self) {
        // println!("{:?}", &self.variables[0..8]);
    }
}

fn exec_int(frame: &mut Frame) {
    let target = frame.read_u8();
    let i = frame.read_u32();
    frame.set_var(target, i);
    frame.dump();
}

fn exec_less_eq(frame: &mut Frame) {
    let ra = frame.read_u8();
    let rb = frame.read_u8();
    let a = frame.var(ra);
    let b = frame.var(rb);
    let target = frame.read_u8();
    frame.set_var(target, (a <= b) as u32);
    frame.dump();
}

fn exec_add(frame: &mut Frame) {
    let ra = frame.read_u8();
    let rb = frame.read_u8();
    let a = frame.var(ra);
    let b = frame.var(rb);
    let target = frame.read_u8();
    frame.set_var(target, a + b);
    frame.dump();
}

fn exec_multiply(frame: &mut Frame) {
    let ra = frame.read_u8();
    let rb = frame.read_u8();
    let a = frame.var(ra);
    let b = frame.var(rb);
    let target = frame.read_u8();
    frame.set_var(target, a * b);
    frame.dump();
}

fn exec_jump_if_not(frame: &mut Frame) {
    let offset = frame.read_u16();
    let source = frame.read_u8();
    let condition = frame.var(source);
    if condition == 0 {
        frame.pc = offset;
    }
}

fn exec_jump(frame: &mut Frame) {
    let offset = frame.read_u16();
    frame.pc = offset;
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
