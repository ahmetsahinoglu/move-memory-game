use std::io::stdin;
use std::thread;
use std::time::Duration;

use rand::{Rng, thread_rng};

const COL: u8 = 6;
const ROW: u8 = 6;
const THREAD_SLEEP_DURATION_SECOND: u64 = 1;

type Area = [[Element; COL as usize]; ROW as usize];

#[derive(Copy, Clone)]
enum Element {
    MONSTER,
    TARGET,
    FOOTPRINT,
    EMPTY,
}

impl Element {
    fn get_icon(&self) -> &str {
        return match self {
            Element::MONSTER => "🐸",
            Element::TARGET => "🍎",
            Element::FOOTPRINT => "🐾",
            Element::EMPTY => "⚪",
        };
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Coordinate {
    x: u8,
    y: u8,
}

trait GameTrait {
    fn start(&mut self) -> ();
    fn show_map(&mut self) -> ();
    fn load_map(&self, hide_icon: bool) -> ();
    fn read_path(&mut self) -> ();
    fn update_monster_position(&mut self, direction: Direction) -> ();
    fn can_update_position(&mut self, direction: Direction) -> bool;
    fn check_result(&mut self, monster_first_y: u8, monster_first_x: u8) -> ();
}

struct Game {
    area: Area,
    monster: Coordinate,
    target: Coordinate,
    score: u32,
}

impl GameTrait for Game {
    fn start(&mut self) -> () {
        let (row, col) = (self.area.len() as u8, self.area[0].len() as u8);

        if self.monster.x >= col || self.target.x >= col || self.monster.y >= row || self.target.y >= row {
            panic!("Type location out of area. Please enter the number inside {}x{}", col, row);
        }
        self.area[self.monster.y as usize][self.monster.x as usize] = Element::MONSTER;
        self.area[self.target.y as usize][self.target.x as usize] = Element::TARGET;
        self.show_map();
    }

    fn show_map(&mut self) -> () {
        self.load_map(false);
        thread::sleep(Duration::from_millis(THREAD_SLEEP_DURATION_SECOND * 1000));
        self.load_map(true);
        self.read_path();
    }

    fn load_map(&self, hide_icon: bool) -> () {
        println!("===========================");
        print!("\x1B[2J\x1B[1;1H");

        for y in 0..self.area.len() {
            for x in 0..self.area[y].len() {
                let element = if hide_icon { Element::EMPTY } else { self.area[y][x] as Element };
                print!(" {} ", element.get_icon());
            }
            println!();
        }

        println!("SKOR: {}", self.score);
    }

    fn read_path(&mut self) -> () {
        println!("\nPlease enter your path with these keys ('w', 's', 'a', 'd'):\n\
          'w' => UP\n\
          's' => DOWN \n\
          'a' => LEFT \n\
          'd' => RIGHT");

        let mut input = String::new();
        if let Ok(_) = stdin().read_line(&mut input) {
            let (monster_first_y, monster_first_x) = (self.monster.y, self.monster.x);

            input = String::from(input.trim());
            for char in input.chars() {
                match char.to_string().as_str() {
                    "w" => self.update_monster_position(Direction::UP),
                    "s" => self.update_monster_position(Direction::DOWN),
                    "a" => self.update_monster_position(Direction::LEFT),
                    "d" => self.update_monster_position(Direction::RIGHT),
                    _ => println!("Unrecognized command!"),
                }
            }
            self.check_result(monster_first_y, monster_first_x);
        }
    }

    fn update_monster_position(&mut self, direction: Direction) -> () {
        let (x, y) = (self.monster.x, self.monster.y);

        match direction {
            Direction::UP => if self.can_update_position(direction) { self.monster.y = y - 1 },
            Direction::DOWN => if self.can_update_position(direction) { self.monster.y = y + 1 },
            Direction::LEFT => if self.can_update_position(direction) { self.monster.x = x - 1 },
            Direction::RIGHT => if self.can_update_position(direction) { self.monster.x = x + 1 },
        }
    }

    fn can_update_position(&mut self, direction: Direction) -> bool {
        let (row, col) = (self.area.len() as u8, self.area[0].len() as u8);

        match direction {
            Direction::UP => self.monster.y > 0,
            Direction::DOWN => self.monster.y < row - 1,
            Direction::LEFT => self.monster.x > 0,
            Direction::RIGHT => self.monster.x < col - 1,
        }
    }

    fn check_result(&mut self, monster_first_y: u8, monster_first_x: u8) -> () {
        if self.monster.x == self.target.x && self.monster.y == self.target.y {
            self.score += 1;

            self.area[monster_first_y as usize][monster_first_x as usize] = Element::EMPTY;
            self.area[self.target.y as usize][self.target.x as usize] = Element::EMPTY;
            self.area[self.monster.y as usize][self.monster.x as usize] = Element::MONSTER;

            let mut empty_coordinates: Vec<Coordinate> = vec![];

            for y in 0..self.area.len() {
                for (x, element) in self.area[y].iter().enumerate() {
                    if let Element::EMPTY = element {
                        empty_coordinates.push(Coordinate {
                            x: x as u8,
                            y: y as u8,
                        });
                    }
                }
            }

            let new_target_position = &empty_coordinates[thread_rng().gen_range(0, empty_coordinates.len()) as usize];
            self.area[new_target_position.y as usize][new_target_position.x as usize] = Element::TARGET;
            self.target.x = new_target_position.x;
            self.target.y = new_target_position.y;

            self.show_map();
        } else {
            self.area[self.monster.y as usize][self.monster.x as usize] = Element::FOOTPRINT;
            self.load_map(false);
            println!("GAME OVER :(");
        }
    }
}


fn main() {
    Game {
        area: [[Element::EMPTY; COL as usize]; ROW as usize],
        monster: Coordinate {
            x: thread_rng().gen_range(0, COL),
            y: thread_rng().gen_range(0, ROW),
        },
        target: Coordinate {
            x: thread_rng().gen_range(0, COL),
            y: thread_rng().gen_range(0, ROW),
        },
        score: 0,
    }.start();
}