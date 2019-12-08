use std::io::BufRead;
use std::io::Write;
use std::iter::FromIterator;
pub trait Interpreter {
    fn run(&mut self) -> bool;
    fn halted(&self) -> bool;
}

pub struct Context {
    ip: usize,
    break_on_output: bool,
    pub data: Vec<i32>,
}

impl Context {
    pub fn new(data: Vec<i32>) -> Self {
        Context {
            ip: 0,
            break_on_output: false,
            data: data,
        }
    }
    pub fn break_on_output(mut self) -> Self {
        self.break_on_output = true;
        self
    }
}

pub struct Io<'a> {
    input: &'a mut dyn std::io::BufRead,
    output: &'a mut dyn std::io::Write,
}
impl<'a> Io<'a> {
    pub fn default(
        input: &'a mut dyn std::io::BufRead,
        output: &'a mut dyn std::io::Write,
    ) -> Self {
        Io {
            input: input,
            output: output,
        }
    }
}

impl Interpreter for (&mut Context, &mut Io<'_>) {
    fn run(&mut self) -> bool {
        let (context, io) = self;
        let data = &mut context.data;

        while context.ip < data.len() {
            let opcode = data[context.ip] % 100;
            let mut modes = vec![false; 0];
            let mut modenum = data[context.ip] / 100;
            // println!("opcode {}", opcode);

            while modenum != 0 {
                modes.push(modenum % 10 == 1);
                modenum /= 10;
            }
            // if !modes.is_empty() {
            //     println!("modes {:?}", modes);
            // }
            if modes.len() < 3 {
                modes.resize(3, false);
            }
            match opcode {
                // ==================== ALU
                1 | 2 | 7 | 8 => {
                    let a = if !modes[0] {
                        &data[data[context.ip + 1] as usize]
                    } else {
                        &data[context.ip + 1]
                    };
                    let b = if !modes[1] {
                        &data[data[context.ip + 2] as usize]
                    } else {
                        &data[context.ip + 2]
                    };
                    if modes[2] {
                        panic!("bad output mode 1");
                    }
                    let c = data[context.ip + 3] as usize;
                    match opcode {
                        1 => data[c] = *a + *b,
                        2 => data[c] = *a * *b,
                        7 => data[c] = (*a < *b).into(),
                        8 => data[c] = (*a == *b).into(),

                        _ => panic!("bad opcode {}", opcode),
                    }
                    context.ip += 4;
                }
                // ==================== input
                3 => {
                    let mut input: String = "".into();
                    io.input.read_line(&mut input);
                    let input = input.trim().parse::<i32>().unwrap();
                    // println!("input: {}", input);

                    let c = data[context.ip + 1] as usize;
                    data[c] = input;
                    context.ip += 2;
                }
                // ==================== output
                4 => {
                    let a = if !modes[0] {
                        &data[data[context.ip + 1] as usize]
                    } else {
                        &data[context.ip + 1]
                    };
                    writeln!(io.output, "{}", *a);
                    context.ip += 2;
                    if context.break_on_output {
                        break;
                    }
                }
                // ==================== cond jump
                5 | 6 => {
                    let a = if !modes[0] {
                        &data[data[context.ip + 1] as usize]
                    } else {
                        &data[context.ip + 1]
                    };
                    let b = if !modes[1] {
                        &data[data[context.ip + 2] as usize]
                    } else {
                        &data[context.ip + 2]
                    };
                    let do_jump = match opcode {
                        5 => *a != 0,
                        6 => *a == 0,
                        _ => panic!("bad opcode {}", opcode),
                    };
                    if do_jump {
                        context.ip = *b as usize;
                    // println!("jump to {}", context.ip);
                    } else {
                        context.ip += 3;
                    }
                }
                // ==================== break
                99 => {
                    break;
                }
                _ => panic!("bad opcode {}", opcode),
            }
        }
        if context.ip >= context.data.len() {
            panic!("ip out of range");
        }
        // println!("finished: {}", finished);
        self.halted()
    }

    fn halted(&self) -> bool {
        let (context, _) = self;

        return context.data[context.ip] == 99;
    }
}
