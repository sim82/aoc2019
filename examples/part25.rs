use aoc2019::intcode::*;

fn main() {
    let code = read_prog("examples/code25.int");
    let mut context = Context::new(code).break_on_output();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut io = IoAscii::default(&mut stdin, &mut stdout);

    loop {
        (&mut context, &mut io as &mut dyn Io2).run();
    }
}
