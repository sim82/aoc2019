use aoc2019::monitoring::*;
use pathfinding::prelude::dijkstra;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    pos: Point,
    goal: char,
    // keys: Vec<char>,
}

struct Graph {
    transitions: Vec<Vec<char>>,
    nodes: Vec<(Point, char)>
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
        let mut nodes = Vec::new();
        for (y, line) in transitions.iter().enumerate() {
            for (x, c) in line.iter().enumerate().filter_map(|(x,c)| if *c == '@' || c.is_lowercase() || c.is_uppercase() { Some((x, c)) } else {None}) {
                nodes.push((Point::new(x as i32, y as i32), *c)); 
            }
        }

        Graph {
            transitions,
            nodes,
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

    fn can_move_to(&self, state: &State, dir: Dir) -> bool {
        let pos = state.pos.move_into(&dir);
        let field = self.get(pos);
        
        match field {
            '.' => true,
            x if x == state.goal => true,    
            _ => false,
        }
    }
    fn move_to(&self, state: &State, dir: Dir) -> (State, usize) {
        State{ state.pos.move_into(&dir), 1};
    }

    fn successors(&self, state: &State) -> Vec<(State, usize)> {
        let mut succ = Vec::new();
        if self.can_move_to(state, Dir::Up) {
            succ.push(self.move_to(state, Dir::Up))
        }
        if self.can_move_to(state, Dir::Down) {
            succ.push(self.move_to(state, Dir::Down))
        }
        if self.can_move_to(state, Dir::Left) {
            succ.push(self.move_to(state, Dir::Left))
        }
        if self.can_move_to(state, Dir::Right) {
            succ.push(self.move_to(state, Dir::Right))
        }
        succ
    }
}

fn main() {
    let data = data18();

    let data: Vec<Vec<char>> = data18().iter().map(|line| line.chars().collect()).collect();
    let graph = Graph::new(data);

    let init_state = State {
        pos: graph.start.clone(),
        keys: Vec::new(),
    };
    let res = dijkstra(
        &init_state,
        |state| {
            let succ = graph.successors(state);
            succ
        },
        |state| state.keys.len() == graph.num_keys,
    );
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
