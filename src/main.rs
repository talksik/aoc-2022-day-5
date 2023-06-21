use std::collections::HashMap;

// enum that converts string letter to enum rep
// all capital letters in alphabet
#[derive(Debug, Copy, Clone)]
enum Crate {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl From<char> for Crate {
    fn from(value: char) -> Self {
        match value {
            'A' => Crate::A,
            'B' => Crate::B,
            'C' => Crate::C,
            'D' => Crate::D,
            'E' => Crate::E,
            'F' => Crate::F,
            'G' => Crate::G,
            'H' => Crate::H,
            'I' => Crate::I,
            'J' => Crate::J,
            'K' => Crate::K,
            'L' => Crate::L,
            'M' => Crate::M,
            'N' => Crate::N,
            'O' => Crate::O,
            'P' => Crate::P,
            'Q' => Crate::Q,
            'R' => Crate::R,
            'S' => Crate::S,
            'T' => Crate::T,
            'U' => Crate::U,
            'V' => Crate::V,
            'W' => Crate::W,
            'X' => Crate::X,
            'Y' => Crate::Y,
            'Z' => Crate::Z,
            _ => panic!("Invalid crate letter"),
        }
    }
}

// the stack number to the crates in that stack
// crates are in order from bottom to top
type Stacks = HashMap<u32, Vec<Crate>>;

fn get_stacks() -> Stacks {
    let mut stacks: Stacks = HashMap::new();

    stacks.insert(
        1,
        vec![
            Crate::F,
            Crate::D,
            Crate::B,
            Crate::Z,
            Crate::T,
            Crate::J,
            Crate::R,
            Crate::N,
        ],
    );
    stacks.insert(2, vec![Crate::R, Crate::S, Crate::N, Crate::J, Crate::H]);
    stacks.insert(
        3,
        vec![
            Crate::C,
            Crate::R,
            Crate::N,
            Crate::J,
            Crate::G,
            Crate::Z,
            Crate::F,
            Crate::Q,
        ],
    );
    stacks.insert(
        4,
        vec![
            Crate::F,
            Crate::V,
            Crate::N,
            Crate::G,
            Crate::R,
            Crate::T,
            Crate::Q,
        ],
    );
    stacks.insert(5, vec![Crate::L, Crate::T, Crate::Q, Crate::F]);
    stacks.insert(
        6,
        vec![
            Crate::Q,
            Crate::C,
            Crate::W,
            Crate::Z,
            Crate::B,
            Crate::R,
            Crate::G,
            Crate::N,
        ],
    );
    stacks.insert(
        7,
        vec![
            Crate::F,
            Crate::C,
            Crate::L,
            Crate::S,
            Crate::N,
            Crate::H,
            Crate::M,
        ],
    );
    stacks.insert(
        8,
        vec![Crate::D, Crate::N, Crate::Q, Crate::M, Crate::T, Crate::J],
    );
    stacks.insert(9, vec![Crate::P, Crate::G, Crate::S]);

    stacks
}

#[derive(Debug)]
struct Move {
    number_crates: u32,
    from_stack: u32,
    to_stack: u32,
}

fn read_moves() -> Vec<Move> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut moves: Vec<Move> = Vec::new();

    for line in input.lines().skip(10) {
        let mut split = line.split_whitespace();
        split.next().unwrap(); // "move"
        let number_crates = split.next().unwrap().parse::<u32>().unwrap();
        split.next().unwrap(); // "from"
        let from_stack = split.next().unwrap().parse::<u32>().unwrap();
        split.next().unwrap(); // "to"
        let to_stack = split.next().unwrap().parse::<u32>().unwrap();

        moves.push(Move {
            number_crates,
            from_stack,
            to_stack,
        });
    }

    moves
}

// part 1: get the last crate in each stack
fn perform_moves(stacks: &mut Stacks, moves: Vec<Move>) {
    for m in moves {
        let from_stack_crates = stacks.get_mut(&m.from_stack).unwrap();
        // remove the last m.number_crates from the stack
        let mut crates_to_move =
            from_stack_crates.split_off(from_stack_crates.len() - m.number_crates as usize);

        // reverse the crates so that the first one taken is on the bottom
        crates_to_move.reverse();

        // add the crates to the to_stack
        let to_stack_crates = stacks.get_mut(&m.to_stack).unwrap();
        to_stack_crates.append(&mut crates_to_move);
    }
}

// do not use, arbitrary order
fn get_the_last_one_in_each_stack(stacks: &Stacks) -> Vec<&Crate> {
    let mut last_crates: Vec<&Crate> = Vec::new();

    for (stack, crates) in stacks {
        println!(
            "stack: {:?}, last_crate: {:?}",
            stack,
            crates.last().unwrap()
        );
        last_crates.push(crates.last().unwrap());
    }

    last_crates
}

fn main() {
    println!("Hello, world!");

    let moves = read_moves();
    let mut stacks = get_stacks();
    println!("before moves: {:?}", stacks);

    perform_moves(&mut stacks, moves);
    get_the_last_one_in_each_stack(&stacks);
    // QNNTGTPVN
}

#[cfg(test)]
mod tests {
    use super::*;

    // todo: finish writing helper to test equality
    #[test]
    fn test() {
        let mut stacks = HashMap::new();
        stacks.insert(1, vec![Crate::Z, Crate::N]);
        stacks.insert(2, vec![Crate::M, Crate::C, Crate::D]);
        stacks.insert(3, vec![Crate::P]);

        let moves = vec![
            Move {
                number_crates: 1,
                from_stack: 2,
                to_stack: 1,
            },
            Move {
                number_crates: 3,
                from_stack: 1,
                to_stack: 3,
            },
            Move {
                number_crates: 2,
                from_stack: 2,
                to_stack: 1,
            },
            Move {
                number_crates: 1,
                from_stack: 1,
                to_stack: 2,
            },
        ];

        println!("before moves {:?}", &stacks);
        perform_moves(&mut stacks, moves);
        eprintln!("after moves {:?}", &stacks);

        assert_eq!(1, 1);
    }
}
