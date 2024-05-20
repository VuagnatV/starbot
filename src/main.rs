use actix::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 20;
const NOISE_SCALE: f64 = 0.1;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Unknown,
    Empty,
    Base,
    Obstacle,
    Resource,
    Robot(usize),
}

impl Cell {
    fn to_char(self) -> char {
        match self {
            Cell::Unknown => '?',
            Cell::Empty => '.',
            Cell::Base => 'B',
            Cell::Obstacle => '#',
            Cell::Resource => 'R',
            Cell::Robot(id) => char::from_digit(id as u32, 10).unwrap_or('X'),
        }
    }
}

#[derive(Clone)]
struct Map {
    grid: Arc<Mutex<Vec<Vec<Cell>>>>,
    robots: Arc<Mutex<HashMap<usize, (usize, usize)>>>,
}

impl Map {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();
        let perlin = Perlin::new().set_seed(seed);
        let mut grid = vec![vec![Cell::Empty; MAP_WIDTH]; MAP_HEIGHT];
        let robots = Arc::new(Mutex::new(HashMap::new()));

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if x == 0 || y == 0 || x == MAP_WIDTH - 1 || y == MAP_HEIGHT - 1 {
                    grid[y][x] = Cell::Obstacle;
                } else {
                    let noise_value = perlin.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]);
                    if noise_value > 0.2 {
                        grid[y][x] = Cell::Obstacle;
                    }
                }
            }
        }

        let (base_x, base_y) = loop {
            let x = rng.gen_range(1..MAP_WIDTH - 1);
            let y = rng.gen_range(1..MAP_HEIGHT - 1);
            if grid[y][x] == Cell::Empty {
                break (x, y);
            }
        };
        grid[base_y][base_x] = Cell::Base;

        let (resource_x, resource_y) = loop {
            let x = rng.gen_range(1..MAP_WIDTH - 1);
            let y = rng.gen_range(1..MAP_HEIGHT - 1);
            if grid[y][x] == Cell::Empty {
                break (x, y);
            }
        };
        grid[resource_y][resource_x] = Cell::Resource;

        Map {
            grid: Arc::new(Mutex::new(grid)),
            robots,
        }
    }

    fn display(&self) {
        clear_terminal();
        let grid = self.grid.lock().unwrap();
        let robots = self.robots.lock().unwrap();
        let mut display_grid = grid.clone();
        for (id, position) in robots.iter() {
            display_grid[position.1][position.0] = Cell::Empty;
            display_grid[position.1][position.0] = Cell::Robot(*id);
        }
        let mut map_display = String::new();
        for row in display_grid.iter() {
            for &cell in row.iter() {
                let _ = write!(map_display, "{} ", cell.to_char());
            }
            let _ = write!(map_display, "\n");
        }
        print!("{}\n", map_display);
    }

    fn update_position(&self, id: usize, new_pos: (usize, usize)) {
        let mut robots = self.robots.lock().unwrap();
        robots.insert(id, new_pos);
    }

    fn is_empty_or_walkable(&self, pos: (usize, usize)) -> bool {
        let grid = self.grid.lock().unwrap();
        grid[pos.1][pos.0] == Cell::Empty
            || grid[pos.1][pos.0] == Cell::Resource
            || grid[pos.1][pos.0] == Cell::Base
    }

    fn get_cell(&self, pos: (usize, usize)) -> Cell {
        let grid = self.grid.lock().unwrap();
        grid[pos.1][pos.0]
    }

    fn find_base_position(&self) -> Option<(usize, usize)> {
        let grid = self.grid.lock().unwrap();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if grid[y][x] == Cell::Base {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

struct Robot {
    id: usize,
    position: (usize, usize),
    map: Map,
    personal_map: Vec<Vec<Cell>>,
    base_map: Arc<Mutex<Vec<Vec<Cell>>>>,
}

impl Robot {
    fn new(id: usize, map: Map, base_map: Arc<Mutex<Vec<Vec<Cell>>>>) -> Self {
        let position = map.find_base_position().unwrap();
        map.update_position(id, position);
        let mut personal_map = vec![vec![Cell::Unknown; MAP_WIDTH]; MAP_HEIGHT];
        personal_map[position.1][position.0] = Cell::Base;
        Robot {
            id,
            position,
            map,
            personal_map,
            base_map,
        }
    }

    fn update_personal_map(&mut self, pos: (usize, usize)) {
        let cell = self.map.get_cell(pos);
        self.personal_map[pos.1][pos.0] = cell;
    }

    fn merge_maps(&mut self) {
        let base_map = self.base_map.lock().unwrap();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if self.personal_map[y][x] == Cell::Unknown && base_map[y][x] != Cell::Unknown {
                    self.personal_map[y][x] = base_map[y][x];
                }
            }
        }
    }

    fn update_base_map(&self) {
        let mut base_map = self.base_map.lock().unwrap();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if base_map[y][x] == Cell::Unknown && self.personal_map[y][x] != Cell::Unknown {
                    base_map[y][x] = self.personal_map[y][x];
                }
            }
        }
    }

    fn display_base_map(&self) {
        //clear_terminal();
        let base_map = self.base_map.lock().unwrap();
        let mut map_display = String::new();
        for row in base_map.iter() {
            for &cell in row.iter() {
                let _ = write!(map_display, "{} ", cell.to_char());
            }
            let _ = write!(map_display, "\n");
        }
        print!("{}\n", map_display);
    }
}

impl Actor for Robot {
    type Context = Context<Self>;
}

struct Start;

impl Message for Start {
    type Result = ();
}

impl Handler<Start> for Robot {
    type Result = ();

    fn handle(&mut self, _msg: Start, ctx: &mut Self::Context) -> Self::Result {
        ctx.run_interval(Duration::from_secs(1), move |act, _| {
            let mut rng = rand::thread_rng();
            let direction = rng.gen_range(0..4);
            let (dx, dy) = match direction {
                0 => (0, 1),
                1 => (0, -1),
                2 => (1, 0),
                _ => (-1, 0),
            };

            let new_position = (
                ((act.position.0 as isize + dx).rem_euclid(MAP_WIDTH as isize)) as usize,
                ((act.position.1 as isize + dy).rem_euclid(MAP_HEIGHT as isize)) as usize,
            );

            if act.map.is_empty_or_walkable(new_position) {
                act.map.update_position(act.id, new_position);
                act.position = new_position;
            }

            if act.map.grid.lock().unwrap()[new_position.1][new_position.0] == Cell::Base {
                act.update_base_map();
                act.merge_maps();
            }

            act.update_personal_map(new_position);

            act.map.display();
            //act.display_base_map();
        });
    }
}

#[actix_rt::main]
async fn main() {
    let num_robots = 5;
    let map = Map::new();
    let base_map = Arc::new(Mutex::new(vec![vec![Cell::Unknown; MAP_WIDTH]; MAP_HEIGHT]));
    let base_position = map.find_base_position().unwrap();
    base_map.lock().unwrap()[base_position.1][base_position.0] = Cell::Base;
    let mut robots = vec![];

    for id in 1..=num_robots {
        let robot = Robot::new(id, map.clone(), base_map.clone()).start();
        robot.do_send(Start);
        robots.push(robot);
    }

    actix_rt::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
    println!("Shutting down...");
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);
}
