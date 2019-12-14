use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::io::{stdin, BufRead};
use std::ops::Mul;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Term {
    element: String,
    num: i32,
}

impl Term {
    pub fn new(element: &str, num: i32) -> Self {
        Term {
            element: element.into(),
            num,
        }
    }
    pub fn from_str(s: &str) -> Self {
        let mut s = s.trim().split(" ");
        Term {
            num: s.next().unwrap().parse::<i32>().unwrap(),
            element: s.next().unwrap().into(),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{} {}'", self.num, self.element)
    }
}

impl Mul<i32> for Term {
    type Output = Term;
    fn mul(self, other: i32) -> Term {
        Term::new(&self.element, self.num * other)
    }
}

impl Mul<i32> for &Term {
    type Output = Term;
    fn mul(self, other: i32) -> Term {
        Term::new(&self.element, self.num * other)
    }
}

trait Reactions {
    fn requirements(&self, output: &Term) -> (Vec<Term>, Term);
}

impl Reactions for HashMap<Term, Vec<Term>> {
    fn requirements(&self, output: &Term) -> (Vec<Term>, Term) {
        println!("want {}", output);
        for kv in self.iter() {
            if kv.0.element == output.element {
                let num_reactions =
                    output.num / kv.0.num + if output.num % kv.0.num != 0 { 1 } else { 0 };
                return (
                    kv.1.iter().map(|x| x * num_reactions).collect(),
                    kv.0 * num_reactions,
                );
            }
        }
        panic!("reaction not found");
    }
}
fn only_ore(terms: &Vec<Term>) -> bool {
    terms.iter().filter(|x| x.element != "ORE").count() == 0
}
fn squash_terms(terms: &Vec<Term>) -> Vec<Term> {
    let mut m = HashMap::<String, i32>::new();
    for term in terms.iter() {
        match m.get_mut(&term.element) {
            Some(mt) => {
                *mt += term.num;
                ()
            }
            None => {
                m.insert(term.element.clone(), term.num);
                ()
            }
        }
    }
    m.iter().map(|kv| Term::new(kv.0, *kv.1)).collect()
}
fn main() {
    let mut reactions = HashMap::new();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let mut split = line.split("=>"); //.collect();
        let input = split.next().unwrap();
        let output = Term::from_str(split.next().unwrap());
        let input_elements: Vec<_> = input.split(",").map(|x| Term::from_str(&x[..])).collect();

        reactions.insert(output, input_elements);
    }
    println!("{:?}", reactions);

    let mut queue = VecDeque::new();
    queue.push_back((Term::new("FUEL", 1), "".to_string()));
    let mut input: Vec<Term> = reactions.requirements(&Term::new("FUEL", 1)).0;
    let mut output: Vec<Term> = vec![Term::new("FUEL", 1)];

    loop {
        input = input
            .iter()
            .flat_map(|x| {
                if x.element == "ORE" {
                    return vec![x.clone()];
                } else {
                    let (i, o) = reactions.requirements(x);
                    println!("add out {}: ({:?}) -> ({})", x, i, o);
                    if o.num > x.num {
                        output.push(Term::new(&x.element, o.num - x.num));
                    }
                    return i;
                }
            })
            .collect();

        input = squash_terms(&input);
        output = squash_terms(&output);

        for in_term in input.iter_mut() {
            for out_term in output.iter_mut() {
                if out_term.element == in_term.element {
                    let min = out_term.num.min(in_term.num);
                    out_term.num -= min;
                    in_term.num -= min;
                    break;
                }
            }
        }
        if only_ore(&input) {
            break;
        }
    }

    println!("input: {:?}", input);
    println!("output: {:?}", output);
}
