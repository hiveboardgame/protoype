use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use crate::position::Position;
use crate::piece::Piece;
use crate::color::Color;
use crate::bug::Bug;

pub struct Board {
    board: HashMap<Position, Vec<Piece>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut positions = self.board.keys().cloned().collect::<Vec<Position>>();
        positions.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        let min_x = positions
            .iter()
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap_or(&Position(0, 0))
            .0;
        let max_x = positions
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap_or(&Position(0, 0))
            .0;
        let min_y = positions
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or(&Position(0, 0))
            .1;
        let max_y = positions
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or(&Position(0, 0))
            .1;
        let mut s = "".to_string();
        for y in min_y..=max_y {
            if y.rem_euclid(2) == 1 {
                write!(s, "{}", "  ")?;
            }
            for x in min_x..=max_x {
                match self.board.get(&Position(x, y)) {
                    Some(piece) => write!(s, "{} ", piece.last().unwrap())?,
                    None => write!(s, "{}", "    ")?,
                };
            }
            write!(s, "{}", "\n")?;
        }
        write!(f, "{}", s)
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: HashMap::new(),
        }
    }

    pub fn neighbor_positions(&self, position: &Position) -> Vec<Position> {
        return vec![
            Position(position.0 - 1, position.1 - 1), // North West
            Position(position.0, position.1 - 1),     // North East
            Position(position.0 + 1, position.1),     // East
            Position(position.0, position.1 + 1),     // South East
            Position(position.0 - 1, position.1 + 1), // South West
            Position(position.0 - 1, position.1),     // West
        ];
    }

    pub fn neighbors(&self, position: &Position) -> Vec<Vec<Piece>> {
        return self
            .neighbor_positions(&position)
            .iter()
            .filter_map(|pos| self.board.get(&pos))
            .cloned()
            .collect();
    }

    pub fn top_layer_neighbors(&self, position: &Position) -> Vec<Piece> {
        return self
            .neighbor_positions(&position)
            .iter()
            .filter_map(|pos| self.board.get(&pos).and_then(|v| v.last()))
            .cloned()
            .collect();
    }

    pub fn negative_space(&self) -> Vec<Position> {
        let taken = self.board.keys().cloned().collect::<HashSet<Position>>();
        let mut all_neighbors = HashSet::new();
        for pos in taken.iter() {
            for pos in self.neighbor_positions(pos) {
                all_neighbors.insert(pos);
            }
        }
        all_neighbors
            .difference(&taken)
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn spawnable(&self, color: Color, position: &Position) -> bool {
        if self.board.keys().len() < 2 {
            return true;
        }
        !self
            .top_layer_neighbors(position)
            .iter()
            .map(|piece| piece.color)
            .collect::<Vec<Color>>()
            .contains(&color.opposite())
    }

    pub fn spawn(&mut self, position: &Position, bug: Bug, color: Color, order: i8) {
        let piece = Piece::new(bug, color, order);
        self.board
            .entry(position.clone())
            .and_modify(|v| v.push(piece.clone()))
            .or_insert(vec![piece]);
    }
}
