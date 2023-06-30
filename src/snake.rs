/*logic to create snake */

use std::collections::LinkedList; //linkedlist allows pushing and poping elements at either end
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;
/*Red, Green, Blue, Opacity */
const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];
#[derive(Copy, Clone, PartialEq)]
pub enum Direction { //handles the direction of snake and how our keyboard inputs interact with snake
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction, //the direction that snake currently travelling
    body: LinkedList<Block>,
    tail: Option<Block>, //we want tail to be actual value when snake eat apple
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        //push_back: appends an element to the back of the list
        //here are setting default snake with length of three
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x: x,
            y,
        });

        Snake {
            direction: Direction::Right,
            body, //means body equals body
            tail: None,
        }
    }
    /*draw snake */
    pub fn draw (&self, con: &Context, g: &mut G2d){
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap(); //return a reference to the front of linkedlist
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (), 
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block); //basically we are removing from the end of snake and add to the front of the snake
        let remove_block = self.body.pop_back().unwrap();
        self.tail = Some(remove_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) : (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => (),
        } 
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }
    /*if snake eat an appkle, this fn is called and the tail is added to the end of body
    This is the method caused snake growing in size */
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return  true;
            }
            ch += 1;
            if ch == self.body.len() -1 {
                break;
            }
        }
        return false;
    }
}

