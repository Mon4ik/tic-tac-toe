use crate::pos2::Pos2;
use crate::utils::{TTTSide, TTTSymbol};

pub struct Engine {}

impl Engine {
    pub fn calculate_winner<'a>(stack: &Vec<TTTSymbol>) -> (&'a str, Vec<Pos2>) {
        let mut map = vec![vec![" "; 3]; 3];

        for sym in stack {
            let str = if sym.side == TTTSide::Cross { "X" } else { "O" };

            map[sym.pos.y][sym.pos.x] = str
        }

        // -- 0;1;2;
        // 0. [][][]
        // 1. [][][]
        // 2. [][][]

        if let Some(winner) = Self::symbols_equal(
            map[0][0],
            map[0][1],
            map[0][2],
        ) {
            (winner, vec![Pos2::new(0, 0), Pos2::new(1, 0), Pos2::new(2, 0)])
        } else if let Some(winner) = Self::symbols_equal(
            map[1][0],
            map[1][1],
            map[1][2],
        ) {
            (winner, vec![Pos2::new(0, 1), Pos2::new(1, 1), Pos2::new(2, 1)])
        } else if let Some(winner) = Self::symbols_equal(
            map[2][0],
            map[2][1],
            map[2][2],
        ) {
            (winner, vec![Pos2::new(0, 2), Pos2::new(1, 2), Pos2::new(2, 2)])
        } else if let Some(winner) = Self::symbols_equal(
            map[0][0],
            map[1][0],
            map[2][0],
        ) {
            (winner, vec![Pos2::new(0, 0), Pos2::new(0, 1), Pos2::new(0, 2)])
        } else if let Some(winner) = Self::symbols_equal(
            map[0][1],
            map[1][1],
            map[2][1],
        ) {
            (winner, vec![Pos2::new(1, 0), Pos2::new(1, 1), Pos2::new(1, 2)])
        } else if let Some(winner) = Self::symbols_equal(
            map[0][2],
            map[1][2],
            map[2][2],
        ) {
            (winner, vec![Pos2::new(2, 0), Pos2::new(2, 1), Pos2::new(2, 2)])
        } else if let Some(winner) = Self::symbols_equal(
            map[0][0],
            map[1][1],
            map[2][2],
        ) {
            (winner, vec![Pos2::new(0, 0), Pos2::new(1, 1), Pos2::new(2, 2)])
        } else if let Some(winner) = Self::symbols_equal(
            map[0][2],
            map[1][1],
            map[2][0],
        ) {
            (winner, vec![Pos2::new(2, 0), Pos2::new(1, 1), Pos2::new(0, 2)])
        } else {
            (" ", vec![])
        }
    }

    pub fn symbols_equal<'a>(a: &'a str, b: &'a str, c: &'a str) -> Option<&'a str> {
        if a == b && b == c {
            Some(a)
        } else {
            None
        }
    }
}