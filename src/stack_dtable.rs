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
        let x = self.bytecode[self.pc as usize];
        self.pc += 1;
        x
    }

    fn peek_u8(&self) -> u8 {
        self.bytecode[self.pc as usize]
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

    fn push(&mut self, x: u32) {
        self.stack[self.sp] = x;
        self.sp += 1;
    }

    fn pop(&mut self) -> u32 {
        let x = self.stack[self.sp - 1];
        self.sp -= 1;
        x
    }
}

static DISPATCH_TABLE: [fn(&mut Frame); 8] = [
    exec_int,
    exec_let,
    exec_var,
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
        DISPATCH_TABLE[opcode as usize](self);
    }

    fn eval(&mut self) {
        while self.peek_u8() != Opcode::Halt as u8 {
            self.step();
        }
    }

    fn dump(&self) {
        // println!("{:?}", &self.stack[0..self.sp]);
    }
}

fn exec_int(frame: &mut Frame) {
    let i = frame.read_u32();
    frame.push(i);
    frame.dump();
}

fn exec_let(frame: &mut Frame) {
    let i = frame.read_u8();
    let val = frame.pop();
    frame.stack[i as usize] = val;
    frame.dump();
}

fn exec_var(frame: &mut Frame) {
    let i = frame.read_u8();
    frame.push(frame.stack[i as usize]);
    frame.dump();
}

fn exec_less_eq(frame: &mut Frame) {
    let b = frame.pop();
    let a = frame.pop();
    frame.push((a <= b) as u32);
    frame.dump();
}

fn exec_add(frame: &mut Frame) {
    let b = frame.pop();
    let a = frame.pop();
    frame.push(a + b);
    frame.dump();
}

fn exec_multiply(frame: &mut Frame) {
    let b = frame.pop();
    let a = frame.pop();
    frame.push(a * b);
    frame.dump();
}

fn exec_jump_if_not(frame: &mut Frame) {
    let offset = frame.read_u16();
    let condition = frame.pop();
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
