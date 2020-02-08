use crate::draw::{draw_block, draw_rectangle};
use crate::snake::*;

use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

const GAME_COLOR: Color = [0.8, 0.8, 0.8, 1.0];
const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [120.0 / 255.0, 56.0 / 255.0, 18.0 / 255.0, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.25;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    score: i32,
    hasFood: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    waitingTime: f64,
    gameOver: bool,
}
//1.生成画布
//2.生成snake
//3.生成snake
//3.接受键盘命令 移动snake
//4.判断是否吃掉,snake增长，并加分
//5.判断是否死掉，结束
impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(5, 5),
            hasFood: false,
            width,
            height,
            score: 0,
            gameOver: false,
            waitingTime: 0.0,
            food_x: 10,
            food_y: 10,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
        if self.hasFood {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        self.snake.draw(con, g);

        if self.gameOver {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut food_x = rng.gen_range(1, self.width - 1);
        let mut food_y = rng.gen_range(1, self.height - 1);
        while self.snake.intersect(food_x, food_y) {
            food_x = rng.gen_range(1, self.width - 1);
            food_y = rng.gen_range(1, self.height - 1);
        }
        self.hasFood = true;
        self.food_x = food_x;
        self.food_y = food_y;
    }

    fn check_eat_food(&mut self) {
        let (x, y) = self.snake.front();
        if self.hasFood && x == self.food_x && y == self.food_y {
            self.hasFood = false;
            self.score += 1;
            println!("得分为:{}", self.score);
            self.snake.eat()
        }
    }

    fn check_is_snake_alive(&self) -> bool {
        let (x, y) = self.snake.front();
        if self.snake.self_intersect() {
            return false;
        }
        x > 1 && x < self.width - 1 && y > 1 && y < self.height - 1
    }

    fn update_snake(&mut self, d: Option<Direction>) {
        if self.check_is_snake_alive() {
            self.snake.move_forward(d);
            self.check_eat_food();
        } else {
            self.gameOver = true;
        }
        self.waitingTime = 0.0;
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.gameOver {
            return;
        }
        // println!("键 pressed{:?}", key);
        let d = match key {
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => self.snake.front_direction(),
        };
        let opposite = self.snake.opposite();
        if d == opposite {
            return;
        }
        // println!("按键移动{:?}", d);
        self.update_snake(Some(d));
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waitingTime += delta_time;

        if self.gameOver == true {
            if self.waitingTime > RESTART_TIME {
                self.restart();
            } else {
                return;
            }
        }

        if self.hasFood == false {
            self.add_food();
        }

        if self.waitingTime > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(5, 5);
        self.score = 0;
        self.gameOver = false;
        self.food_x = 10;
        self.food_y = 10;
        self.waitingTime = 0.0;
        self.hasFood = true;
    }
}
