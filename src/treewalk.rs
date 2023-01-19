pub enum Instruction {
    Int(u32),

    Var(u8),
    Let {
        variable: u8,
        value: Box<Instruction>,
    },

    LessEq(Box<Instruction>, Box<Instruction>),
    Add(Box<Instruction>, Box<Instruction>),
    Multiply(Box<Instruction>, Box<Instruction>),

    Sequence(Vec<Instruction>),
    While {
        condition: Box<Instruction>,
        body: Box<Instruction>,
    },
}

struct Frame {
    variables: [u32; 256],
}

impl Frame {
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

fn interpret(frame: &mut Frame, code: &Instruction) -> u32 {
    match code {
        Instruction::Int(i) => *i,

        Instruction::Var(v) => frame.var(*v),
        Instruction::Let { variable, value } => {
            let value = interpret(frame, value);
            frame.set_var(*variable, value);
            value
        }

        Instruction::LessEq(a, b) => (interpret(frame, a) <= interpret(frame, b)) as u32,
        Instruction::Add(a, b) => interpret(frame, a) + interpret(frame, b),
        Instruction::Multiply(a, b) => interpret(frame, a) * interpret(frame, b),

        Instruction::Sequence(s) => {
            let mut last = 0;
            for insn in s {
                last = interpret(frame, insn);
            }
            last
        }
        Instruction::While { condition, body } => {
            let mut last = 0;
            while interpret(frame, condition) != 0 {
                last = interpret(frame, body);
            }
            last
        }
    }
}

const VAR_N: u8 = 0;
const VAR_I: u8 = 1;
const VAR_X: u8 = 2;

pub fn code() -> Instruction {
    use Instruction::*;

    Sequence(vec![
        // i = 1
        Let {
            variable: VAR_I,
            value: Box::new(Int(1)),
        },
        // x = 1
        Let {
            variable: VAR_X,
            value: Box::new(Int(1)),
        },
        // while
        While {
            // i <= n
            condition: Box::new(LessEq(Box::new(Var(VAR_I)), Box::new(Var(VAR_N)))),
            // {
            body: Box::new(Sequence(vec![
                // x = x * i
                Let {
                    variable: VAR_X,
                    value: Box::new(Multiply(Box::new(Var(VAR_X)), Box::new(Var(VAR_I)))),
                },
                // i = i + 1
                Let {
                    variable: VAR_I,
                    value: Box::new(Add(Box::new(Var(VAR_I)), Box::new(Int(1)))),
                },
            ])),
            // }
        },
    ])
}

pub fn run(code: &Instruction) -> u32 {
    let mut frame = Frame {
        variables: [0; 256],
    };
    frame.variables[VAR_N as usize] = 10;
    interpret(&mut frame, &code);
    frame.variables[VAR_X as usize]
}
