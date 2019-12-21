use smallvec::SmallVec;
use std::io::BufRead;
use std::io::Write;
use std::iter::FromIterator;
use std::sync::mpsc::{Receiver, Sender};

pub trait Interpreter {
    fn run(&mut self) -> bool;
    fn run_for(&mut self, steps: i64) -> bool;
    fn halted(&self) -> bool;
}

pub struct Context {
    ip: usize,
    relbase: usize,
    break_on_output: bool,
    pub data: Vec<i64>,
}

impl Context {
    pub fn new(data: Vec<i64>) -> Self {
        Context {
            ip: 0,
            relbase: 0,
            break_on_output: false,
            data: data,
        }
    }
    pub fn reset(&mut self, data: &Vec<i64>) {
        self.ip = 0;
        self.relbase = 0;
        if self.data.len() < data.len() {
            self.data = data.clone();
        } else {
            // self.data.copy_from_slice(&data[..]);
            // println!("reset {} {}", self.data.len(), data.len());
            self.data[..data.len()].copy_from_slice(data);
            if self.data.len() > data.len() {
                for i in &mut self.data[data.len()..] {
                    *i = 0;
                }
            }
        }
    }
    pub fn break_on_output(mut self) -> Self {
        self.break_on_output = true;
        self
    }
    pub fn halted(&self) -> bool {
        self.data[self.ip] == 99
    }
    pub fn check_size(&mut self, addr: i64) {
        if addr < 0 {
            panic!("invalid address {}", addr);
        }
        let addr = addr as usize;
        if addr >= self.data.len() {
            self.data.resize(addr + 1, 0);
        }
    }
    pub fn load(&mut self, mode: i64, offs: usize) -> i64 {
        // println!("load: {} {} {}", self.ip, offs, self.relbase);
        let addr = match mode {
            0 => self.data[self.ip + 1 + offs],
            1 => (self.ip + 1 + offs) as i64,
            2 => self.relbase as i64 + self.data[self.ip + 1 + offs],
            _ => panic!("bad parameter mode"),
        };
        self.check_size(addr);
        // println!("load from: {}", addr);
        self.data[addr as usize]
    }
    pub fn store(&mut self, mode: i64, offs: usize, v: i64) {
        let addr = match mode {
            0 => self.data[self.ip + 1 + offs],
            2 => self.data[self.ip + 1 + offs] + self.relbase as i64,
            _ => panic!("bad parameter mode"),
        };
        self.check_size(addr);
        self.data[addr as usize] = v
    }
}

pub trait Io2 {
    fn read(&mut self) -> i64;
    fn write(&mut self, v: i64);
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
impl<'a> Io2 for Io<'a> {
    fn write(&mut self, v: i64) {
        writeln!(self.output, "{}", v).unwrap();
    }
    fn read(&mut self) -> i64 {
        let mut input: String = "".into();
        self.input.read_line(&mut input).unwrap();
        input.trim().parse::<i64>().unwrap()
    }
}
impl Io2 for (&Sender<i64>, &Receiver<i64>, i64) {
    fn read(&mut self) -> i64 {
        match self.1.try_recv() {
            Ok(i) => {
                // println!("ch({}) read {}", self.2, i);
                i
            }
            Err(err) => panic!("ch({}) read failed: {}", self.2, err),
        }
    }
    fn write(&mut self, v: i64) {
        // println!("ch({}) write {}", self.2, v);

        self.0.send(v).unwrap()
    }
}

impl Io2 for () {
    fn read(&mut self) -> i64 {
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();
        line.trim().parse::<i64>().unwrap()
    }
    fn write(&mut self, v: i64) {
        writeln!(std::io::stdout().lock(), "{}", v).unwrap();
    }
}

pub struct IoAscii<'a> {
    input: &'a mut dyn std::io::BufRead,
    output: &'a mut dyn std::io::Write,
}
impl<'a> IoAscii<'a> {
    pub fn default(
        input: &'a mut dyn std::io::BufRead,
        output: &'a mut dyn std::io::Write,
    ) -> Self {
        IoAscii {
            input: input,
            output: output,
        }
    }
}
impl<'a> Io2 for IoAscii<'a> {
    fn write(&mut self, v: i64) {
        // writeln!(self.output, "{}", v).unwrap();
        if v > 127 {
            writeln!(self.output, "non ascii: {}", v).unwrap();
        } else {
            let buf = [v as u8; 1];
            self.output.write(&buf).unwrap();
        }
    }
    fn read(&mut self) -> i64 {
        let mut buf = [0 as u8; 1];
        self.input.read_exact(&mut buf).unwrap();
        buf[0] as i64
    }
}

pub struct Process {
    pub context: Context,
    pub input: String,
    pub output: Vec<u8>,
    pub break_output: bool,
}

impl Interpreter for (&mut Context, &mut dyn Io2) {
    fn run(&mut self) -> bool {
        self.run_for(-1)
    }
    fn run_for(&mut self, mut steps: i64) -> bool {
        let (context, io) = self;
        // let data = &context.data;
        while steps != 0 && context.ip < context.data.len() {
            steps -= 1;
            let opcode = context.data[context.ip] % 100;
            let mut modes = SmallVec::<[i64; 4]>::new(); // vec![0; 0];
            modes.reserve(3);
            let mut modenum = context.data[context.ip] / 100;
            // println!("opcode {}", opcode);

            while modenum != 0 {
                modes.push(modenum % 10);
                modenum /= 10;
            }
            // if !modes.is_empty() {
            //     println!("modes {:?}", modes);
            // }
            if modes.len() < 3 {
                modes.resize(3, 0);
            }
            match opcode {
                // ==================== ALU
                1 | 2 | 7 | 8 => {
                    let a = context.load(modes[0], 0);
                    let b = context.load(modes[1], 1);
                    match opcode {
                        1 => context.store(modes[2], 2, a + b),
                        2 => context.store(modes[2], 2, a * b),
                        7 => context.store(modes[2], 2, (a < b).into()),
                        8 => context.store(modes[2], 2, (a == b).into()),

                        _ => panic!("bad opcode {}", opcode),
                    }
                    context.ip += 4;
                }
                // ==================== input
                3 => {
                    // let c = context.data[context.ip + 1] as usize;
                    context.store(modes[0], 0, io.read());
                    context.ip += 2;
                }
                // ==================== output
                4 | 9 => {
                    let a = context.load(modes[0], 0);
                    match opcode {
                        4 => {
                            io.write(a);
                            if context.break_on_output {
                                context.ip += 2;
                                break;
                            }
                        }
                        9 => {
                            context.relbase = (context.relbase as isize + a as isize) as usize;
                            // println!("relbase: {}", context.relbase);
                        }
                        _ => panic!("bad opcode {}", opcode),
                    }
                    context.ip += 2;
                }
                // ==================== cond jump
                5 | 6 => {
                    let a = context.load(modes[0], 0);
                    let b = context.load(modes[1], 1);
                    let do_jump = match opcode {
                        5 => a != 0,
                        6 => a == 0,
                        _ => panic!("bad opcode {}", opcode),
                    };
                    if do_jump {
                        context.ip = b as usize;
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
