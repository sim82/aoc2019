use aoc2019::monitoring::*;
use pathfinding::prelude::astar;
use pathfinding::prelude::dijkstra;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    pos: [Point; 4],
    keys: Vec<char>,
}

struct Graph {
    transitions: Vec<Vec<char>>,
    starts: Vec<Point>,
    num_keys: usize,
}

fn to_lowercase_western(c: &char) -> char {
    let c = *c;
    if c as u16 >= 'A' as u16 && c as u16 <= 'Z' as u16 {
        (c as u16 + 32) as u8 as char // meeeeehh...
    } else {
        panic!("not uppercase: {}", c);
    }
}

impl Graph {
    fn new(transitions: Vec<Vec<char>>) -> Graph {
        let mut starts = Vec::new();
        let mut num_keys = 0;
        for (y, line) in transitions.iter().enumerate() {
            // println!("line: {:?}", line);
            for x in line
                .iter()
                .enumerate()
                .filter_map(|(x, c)| if *c == '@' { Some(x) } else { None })
            {
                println!("start: {} {}", x, y);
                starts.push(Point::new(x as i32, y as i32));
            }
            num_keys += line.iter().filter(|x| x.is_lowercase()).count();
        }
        println!("start: {:?}\nnum_keys: {}", starts, num_keys);

        Graph {
            transitions,
            starts,
            num_keys,
        }
    }

    fn get(&self, pos: Point) -> char {
        let x = pos.x as usize;
        let y = pos.y as usize;
        if y >= self.transitions.len() || x >= self.transitions[y].len() {
            panic!("out of bounds: {} {}", x, y);
        }
        self.transitions[y as usize][x as usize]
    }

    fn can_move_to(&self, state: &State, dir: Dir, num: usize) -> bool {
        let pos = state.pos[num].move_into(&dir);
        let field = self.get(pos);
        match field {
            '#' => false,
            '.' | '@' => true,
            x if x.is_uppercase() => {
                let cont = state.keys.contains(&to_lowercase_western(&field));
                // println!(
                //     "cont: {:?} {:?} {}",
                //     state.keys,
                //     to_lowercase_western(&field),
                //     cont,
                // );
                cont
            }
            x if x.is_lowercase() => true,
            _ => panic!("unhandled char: {}", field),
        }
    }
    fn move_to(&self, state: &State, dir: Dir, num: usize) -> (State, usize) {
        let pos = state.pos[num].move_into(&dir);
        let field = self.get(pos);
        let mut new_state = state.clone();

        if field.is_lowercase() {
            //keys.insert(field);
            if new_state.keys.iter().find(|tk| **tk == field).is_none() {
                new_state.keys.push(field);
                new_state.keys.sort_by_key(|x| x.clone());
            }
        }
        new_state.pos[num] = pos;
        (new_state, 1)
    }

    fn successors(&self, state: &State) -> Vec<(State, usize)> {
        let mut succ = Vec::new();
        for i in 0..4 {
            if self.can_move_to(state, Dir::Up, i) {
                succ.push(self.move_to(state, Dir::Up, i))
            }
            if self.can_move_to(state, Dir::Down, i) {
                succ.push(self.move_to(state, Dir::Down, i))
            }
            if self.can_move_to(state, Dir::Left, i) {
                succ.push(self.move_to(state, Dir::Left, i))
            }
            if self.can_move_to(state, Dir::Right, i) {
                succ.push(self.move_to(state, Dir::Right, i))
            }
        }
        succ
    }
}

fn main() {
    let data = data18();

    let data: Vec<Vec<char>> = data18().iter().map(|line| line.chars().collect()).collect();
    let graph = Graph::new(data);

    let init_state = State {
        pos: [
            graph.starts[0],
            graph.starts[1],
            graph.starts[2],
            graph.starts[3],
        ],
        keys: Vec::new(),
    };
    let mut num_calls = 0;
    // let res = astar(
    let res = dijkstra(
        &init_state,
        |state| {
            num_calls += 1;
            // println!("state: {:?}", state);
            let succ = graph.successors(state);
            // println!(" - succ: {:?}", succ);
            succ
        },
        // |state| 26 - state.keys.len(),
        |state| state.keys.len() == graph.num_keys,
    );
    println!("calls {}", num_calls);
    println!("res: {:?}", res);
}

fn data18() -> Vec<&'static str> {
    if true {
        // vec![
        //     "###############",
        //     "#d.ABC.#.....a#",
        //     "######@#@######",
        //     "###############",
        //     "######@#@######",
        //     "#b.....#.....c#",
        //     "###############",
        // ]
        // vec![
        //     "#############",
        //     "#DcBa.#.GhKl#",
        //     "#.###@#@#I###",
        //     "#e#d#####j#k#",
        //     "###C#@#@###J#",
        //     "#fEbA.#.FgHi#",
        //     "#############",
        // ]
        vec![
            "#############",
            "#g#f.D#..h#l#",
            "#F###e#E###.#",
            "#dCba@#@BcIJ#",
            "#############",
            "#nK.L@#@G...#",
            "#M###N#H###.#",
            "#o#m..#i#jk.#",
            "#############",
        ]
    } else {
        vec![
            "#################################################################################",
            "#.....#........q..#...........#.........#.#...#.......#.#...#...#.....P...#.....#",
            "###E#.#####.#######.###G#####.#########.#.#.#.#.###.#.#.#.#.#.#.#.#######.#.#.#.#",
            "#...#.....#...#.....#.#.#i..#.#...J.....#...#.....#.#...#.#...#.#.......#...#.#.#",
            "#.#######.###.#.#####.#.#.###.#.#######.#.#########.###.#.#####.#####.#######.#.#",
            "#.......#.....#.#.....#.#...#...#.....#.#.#.....#...#...#.#...#.#...#.#.......#.#",
            "#.#####.###.###.#.#.###.###.#########.#.#.###.#.#.#.#####.#.###.#.#.#.#.#########",
            "#.....#.#.#.#...#.#...#...#.....#.....#.#...#.#.#.#t#.....#...#...#.#.#.#.......#",
            "#####.#.#.#.#.#######.###.#.###.#.###.#.###.#.###.###.#######.#####.#.#.#.#####.#",
            "#...#.#.#...#.........#...#.#.....#.#.#.#.#...#...#...#...........#.#.#.#.#...#.#",
            "#.#.#.#.###.#########.#.###.#######.#.#.#.#.###.#.#.#######.#####.#.###.#.###.#.#",
            "#.#p#.#...#...#...#...#.#.#.......#...#.#.#.#...#.#.....#...#...#.#.#...#.....#.#",
            "#.#.#.###.###.#.###.###.#.#.#####.#.###.#.#.#.#########.#.###.#.###.#.#####.###.#",
            "#.#...#...#...#...#.#...#...#.#...#.#...#...#.#.......#...#...#.....#.#...#.#...#",
            "#####.#.###.#####.#.#.###.###.#.###.#.###.###.#.###.#####.#.#########.#.#.#.#.#.#",
            "#...#.#r#...#.....#.#...#.....#.#...#...#.#.#.#.#.#.....#.#.#.......#...#...#.#.#",
            "#.#.###.#.#####.#.#.###.#####.#.#.#####.#.#.#.#.#.###.#.#.#.###.###.#.#######.#.#",
            "#.#.....#.#...#.#.#.#.#.....#.#.#...#...#...#...#...#.#.#.#...#.#...#...#.....#.#",
            "#.#######.#.#.#.#.#.#.#####.###.###.#.#####.#####.#.#.#.#.###.#.#.#####.#.#####.#",
            "#.....#.#...#...#.#.......#...#...#.#.#.#.#.#.....#.#.#.#.#.#.#.#.#.....#.#...#.#",
            "#####.#.#####.#####.#####.###Y#.#.#.#.#.#.#.###.#.#.#.###.#.#.#.###.#####.#.#.#.#",
            "#.....#.#.....#...#.#...#...#.#.#.#.#.#.#.#...#.#.#.#...#...#.#.#...#...#.#.#.#.#",
            "#.#####.#.#####.#.#.###.#.###.#.#.###.#.#.###.###.#.###.###.###.#.#####.#.###.#.#",
            "#...#.......#.#.#.....#.#.#...#.#...#...#...#..b..#.#.#.#...#...........#...#...#",
            "#.#.#######.#.#.#######.###.#######.###.#.#########.#.#.#.#####.#######.###.###.#",
            "#.#.#...#.D...#v#.....#...#.#.....#.....#.#.......#...#.#.#...#.#...#.....#...#.#",
            "#.#.#L#.#######.#.###.###.#.#.###.#.#####.#.#.#######.#.#.#.#.#.#.#.#########.#.#",
            "#.#...#.#.....#.#...#.....#.....#.#...#.#.#.#.........#...#.#.#.#.#.#.......#.#.#",
            "#.#####.#.###.#.###.#############.###.#.#.#.###############.#.###.#.#.#####.#.#.#",
            "#.#...#...#.#...#.#.........#.....#...#.#.#.#.#.......#.....#...#.#.#.#.....V.#.#",
            "#.#.#.#####.#####.#########.#.#######.#.#.#.#.#.###X#.#.#.#####.#.#.#.#########.#",
            "#...#.#...#.........#.....#.#.......#...#...#.#...#.#.#.#.#...#.#.#.#.....#.R.#.#",
            "#####.#.###.#.#####.#.#####.#.#####.#####.###.###.#.#.###.#.###.#.#.#.###.#.###.#",
            "#...#.#.....#.....#.#.....#.#.....#.....#.#.......#.#....y#.#...#.#.#.#...#.#...#",
            "#.###.#.#########.#.#####.#.#####.#####.#.###.#####.#######.#.###.#.###.###.#.###",
            "#.....#...#.......#.#.....#...#.....#.#.#...#.....#.#.....#...#...#...#.#.....#.#",
            "#.#########.#######.#.#######.#####.#.#.###.#######.#.###.#.#####.###.#.#.#####.#",
            "#.........#.#.#...#.#.......#.#...#...#.#...#.....#.#.#...#.#...#...#...#.....#.#",
            "#########.#.#.#.#.#.#####.#.#.#.#.#####.#.###.###.#.#.#.###.#.#.###.#########.#.#",
            "#...........#...#.........#.#...#......@#@....#.....#.#.......#.....#...........#",
            "#################################################################################",
            "#.........#.....#...#...........#......@#@....#.......#.......#.............#...#",
            "#.#.#######.#.###.#.###.#######.###.###.###.#.#.#####.#.#.###.#####.#####.#.#.#.#",
            "#.#m#z......#.....#..o#.....#.......#...#...#...#.....#.#...#.#...#...#...#.#.#.#",
            "#.#.#.###############.#####.#########.###.#########.#######.#.#.#.#.###.###.#.#.#",
            "#.#.#.....#.........#.....#.#...#...#.#.#.........#.#.......#...#.#.#...#.#.#.#.#",
            "#.#.#####.###.###.#.#####.#.#.#.#.#.#.#.#.#######.#.###.#########.###.###.#.###.#",
            "#.#.#...#...#...#.#...#...#...#...#.#...#.....#...#.....#...#.....#...#...#...#.#",
            "#.###.#.###.###.#.###.#.###########.###.#######.#########.###.#####.###.#.###.#.#",
            "#.....#.....#...#.#...#...#.....W.#...#.#.......#.........#...#.....#...#...#...#",
            "#.###########.###.#.#####.#.#####.###.#.#.#######.###.#####.###.#.###.###.#####.#",
            "#.....#.........#.#...#...#.#.....#...#.#.#...#.....#.....#.#...#.#...#.#.....#.#",
            "#####.###########.#.###.###.#.#####.###.#.#.#.#.#####.###.#.#####.#.###.#####.#.#",
            "#...#.....#.......#.#...#...#..n#.....#x#.#.#.#.#...#...#.......#.#.#...#...#...#",
            "#.#######.#.#######.#.#.#.#####.#####.#.#.#.#.###.#.###########.#.#.#.#.#.#.#####",
            "#...........#c....#.#.#.#...#.#.#.....#.#...#.#...#...#.......#...#l..#...#.....#",
            "#.###########.###.#.#.#.###.#.#.#.#####.#####.#.#####.#.#####.#################.#",
            "#.........#.F.#.#.#.#.#...#.#.#.#.....#.#...#.#.....#.#...#.#.#.....#.#.......#.#",
            "#########.#.###.#.#.#H###.#.#.#.#.###.###.#.#.#####.#.###.#.#.###.#.#.#.#####.#.#",
            "#.......#.#.....#.#.#...#.#...#.#...#...#.#.......#.#...#.#.#...#.#...#.#...#...#",
            "#.###.###.#####.#.#####.#.###.#.###.###.#.#####.###.#.###T#.###.#.#####.###.#####",
            "#...#.........#.#...#.#.#...#.#.#...#...#...#...#...#.....#.#...#.#.....#.......#",
            "#.#############.###.#C#.#####.#.###.#.#####.#.###.#########.#####.#.#####.#######",
            "#.#........f....#.#...#.......#...#g#...#...#...#.#...............#.#...#......u#",
            "#.#K#############.#######.#######N#.###.#.#####B#.###########.###.#.#.#.#####.#.#",
            "#.#.....#.....A.#....a..#.#.U...#.#...#.#.#.....#...#.........#...#...#.....#.#.#",
            "#.#####.###.###.###.###.#.#.###.#.#.###.#.#########.#.#########.###########.###.#",
            "#.#...#..k..#.#.....#.#.#...#w..#.#.#...#...........#.#...#...#.......#.....#...#",
            "#.#.#########.#######.#.#########.#.#.#.#############.#.#.#.#.#######.#.#####.###",
            "#.#.........#.........#.#......s..#.#.#.#.#.......#...#.#...#.......#.#.#.#.....#",
            "#.#.###.###.#####.###.#S#.#########.#.###.#.#.###.#.###.###########.###.#.#.###.#",
            "#.#...#.#.#.#...#.#...#...#...#...#.#...#.#.#.#.....#...#.....#...#...#.#...#...#",
            "#.#####.#.#.#.#.#.#.#######.#.#.###.###.#.#.#.###.###.###.#.#.###.###.#.#.###.###",
            "#..e....#.#.#.#...#...#.....#.#.......#.#...#...#.#...#...#.#.......#...#...#...#",
            "#########.#.#.#######.#####.#.#.#######.#.#####.#M#.#####.#.#######.###########.#",
            "#.......I.#.#.#.......#.....#.#..j#...#.#...#...#.#.#...#.#...#...#.#...#.......#",
            "#.#########.#.#.#######.#####.#####.#.#.#####.#####Q#.#.#####.###.#.#.#.#.#####.#",
            "#.....#...#...#.......#.#...#.......#.#.#.....#.....#.#.....#...#.#.Z.#.#.O.#.#.#",
            "#.###.#.#.###########.#.#.###########.#.#.#####.#####.#####.###.#.#####.###.#.#.#",
            "#...#...#...............#...............#..d..........#.........#...........#..h#",
            "#################################################################################",
        ]
    }
}
