use std::collections::HashSet;

fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn new(x: i32, y:i32) -> Self {
        Pos {x, y}
    }
}

struct  VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_position: Pos
}

impl VisitedHouses {
    #[must_use]
    fn new() -> Self {
        let mut visited_houses: HashSet<Pos> = HashSet::new();
        visited_houses.insert(Pos::new(0, 0));

        VisitedHouses{
            visited_houses,
            current_position: Pos::new(0, 0)
        }
    }

    #[must_use]
    fn current_pos(&self) -> Pos {
        self.current_position
    }

    fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    fn perform_move(&self, direction:Direction) {
        let new_position: Pos = match direction {
            Direction::North => Pos::new(self.current_position.x,self.current_position.y+1),
            Direction::East => Pos::new(self.current_position.x+1,self.current_position.y),
            Direction::South => Pos::new(self.current_position.x, self.current_position.y-1),
            Direction::West => Pos::new(self.current_position.x-1,self.current_position.y)
        }
        self.current_position = new_position;
        self.visited_houses.insert(new_position);

    }
}
#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let visited_houses = VisitedHouses::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        assert_eq!(visited_houses.current_pos(), Pos{x:0, y:0});
    }

    // #[test]
    // fn test_direction_try_from() {
    //     assert_eq!('^'.try_into(), Ok(Direction::North));
    //     assert_eq!('v'.try_into(), Ok(Direction::South));
    //     assert_eq!('<'.try_into(), Ok(Direction::West));
    //     assert_eq!('>'.try_into(), Ok(Direction::East));
    //     assert_eq!('x'.try_into(), Err(IllegalDirectionCharacter('x')));
    // }

    // #[test]
    // fn test_move_east() {
    //     let mut visited_houses = VisitedHouses::new();
    //     visited_houses.perform_move(Direction::East);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(1, 0));
    // }

    // #[test]
    // fn test_square_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^>v<").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 4);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_up_down_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^v^v^v^v^v").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_aoc_input() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str(include_str!("../input.txt")).unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2565);
    // }
}
