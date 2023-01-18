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

struct State {
    variables: [u32; 8],
}

fn interpret(state: &mut State, code: &Instruction) -> u32 {
    match code {
        Instruction::Int(i) => *i,

        Instruction::Var(v) => state.variables[*v as usize],
        Instruction::Let { variable, value } => {
            let value = interpret(state, value);
            state.variables[*variable as usize] = value;
            value
        }

        Instruction::LessEq(a, b) => (interpret(state, a) <= interpret(state, b)) as u32,
        Instruction::Add(a, b) => interpret(state, a) + interpret(state, b),
        Instruction::Multiply(a, b) => interpret(state, a) * interpret(state, b),

        Instruction::Sequence(s) => {
            let mut last = 0;
            for insn in s {
                last = interpret(state, insn);
            }
            last
        }
        Instruction::While { condition, body } => {
            let mut last = 0;
            while interpret(state, condition) != 0 {
                last = interpret(state, body);
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
    let mut state = State { variables: [0; 8] };
    state.variables[VAR_N as usize] = 10;
    interpret(&mut state, &code);
    state.variables[VAR_X as usize]
}
