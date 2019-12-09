use aoc2019::intcode::*;

use aoc2019::intcode::Io2;

fn main() {
    {
        let mut context = Context::new(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        (&mut context, &mut () as &mut dyn Io2).run();
    }
    // {
    //     let mut context = Context::new(vec![104, 1125899906842624, 99]);
    //     (&mut context, &mut () as &mut dyn Io2).run();
    // }
}
