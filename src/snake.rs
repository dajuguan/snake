//
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

use crate::draw::draw_block;
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.0, 0.0, 1.0, 1.0];

#[derive(Debug, Clone)]
struct BLOCK {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    d: Direction,
    body: LinkedList<BLOCK>,
    tail: Option<BLOCK>,
}
//初始化
//move
//吃东西
//
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body = LinkedList::new();
        body.push_back(BLOCK { x: x + 2, y });
        body.push_back(BLOCK { x: x + 1, y });
        body.push_back(BLOCK { x, y });

        Snake {
            d: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn opposite(&self) -> Direction {
        match self.d {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            let x = block.x;
            let y = block.y;
            draw_block(SNAKE_COLOR, x, y, con, g);
        }
    }
    pub fn front(&self) -> (i32, i32) {
        let front_block = self.body.front().unwrap();
        (front_block.x, front_block.y)
    }

    pub fn front_direction(&self) -> Direction {
        self.d
    }
    pub fn move_forward(&mut self, d: Option<Direction>) {
        match d {
            Some(d) => self.d = d,
            _ => (),
        }
        // print!("move forward{:?}", self.d);
        let (x, y) = self.front();
        let new_block = match self.d {
            Direction::Up => BLOCK { x, y: y - 1 },
            Direction::Down => BLOCK { x, y: y + 1 },
            Direction::Left => BLOCK { x: x - 1, y },
            Direction::Right => BLOCK { x: x + 1, y },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }
    //check if the produced food will intersect with the body
    pub fn intersect(&self, x: i32, y: i32) -> bool {
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }
        }
        false
    }
    fn next_head(&self) -> (i32, i32) {
        let (x, y) = self.front();
        let (x, y) = match self.d {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        (x, y)
    }
    pub fn self_intersect(&self) -> bool {
        let (x, y) = self.next_head();
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            if ch == self.body.len() - 1 {
                return false;
            }
        }
        return false;
    }
    pub fn eat(&mut self) {
        self.body.push_back(self.tail.clone().unwrap());
    }
}
