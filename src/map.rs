use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::sync::{Arc, Mutex};

pub const MAP_WIDTH: usize = 20;
pub const MAP_HEIGHT: usize = 20;
const NOISE_SCALE: f64 = 0.1; // Scale for the Perlin noise

const NUM_MINERALS: usize = 5;
const NUM_ENERGY: usize = 5;
const NUM_SCIENCE_POI: usize = 3;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Unknown,
    Empty,
    Base,
    Obstacle,
    Mineral,
    Energy,
    SciencePOI,
    Robot(usize),
}

impl Cell {
    pub fn to_char(self) -> char {
        match self {
            Cell::Unknown => '?',
            Cell::Empty => '.',
            Cell::Base => 'B',
            Cell::Obstacle => '#',
            Cell::Mineral => 'M',
            Cell::Energy => 'E',
            Cell::SciencePOI => 'S',
            Cell::Robot(id) => char::from_digit(id as u32, 10).unwrap_or('X'),
        }
    }
}

#[derive(Clone)]
pub struct Map {
    pub grid: Arc<Mutex<Vec<Vec<Cell>>>>,
    robots: Arc<Mutex<HashMap<usize, (usize, usize)>>>, // Robot positions
}

impl Map {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();
        let perlin = Perlin::new().set_seed(seed);
        let mut grid = vec![vec![Cell::Empty; MAP_WIDTH]; MAP_HEIGHT];
        let robots = Arc::new(Mutex::new(HashMap::new()));

        // Use Perlin noise to place obstacles inside the map
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if x == 0 || y == 0 || x == MAP_WIDTH - 1 || y == MAP_HEIGHT - 1 {
                    // Place obstacles on the borders
                    grid[y][x] = Cell::Obstacle;
                } else {
                    let noise_value = perlin.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]);
                    if noise_value > 0.2 {
                        grid[y][x] = Cell::Obstacle;
                    }
                }
            }
        }

        // Randomly place the base in a valid position
        let (base_x, base_y) = loop {
            let x = rng.gen_range(1..MAP_WIDTH - 1);
            let y = rng.gen_range(1..MAP_HEIGHT - 1);
            if grid[y][x] == Cell::Empty {
                break (x, y);
            }
        };
        grid[base_y][base_x] = Cell::Base;

        // Function to place resources
        fn place_resources(grid: &mut Vec<Vec<Cell>>, resource: Cell, count: usize) {
            let mut rng = rand::thread_rng();
            let mut placed = 0;
            while placed < count {
                let x = rng.gen_range(1..MAP_WIDTH - 1);
                let y = rng.gen_range(1..MAP_HEIGHT - 1);
                if grid[y][x] == Cell::Empty {
                    grid[y][x] = resource;
                    placed += 1;
                }
            }
        }

        // Place different types of resources
        place_resources(&mut grid, Cell::Mineral, NUM_MINERALS);
        place_resources(&mut grid, Cell::Energy, NUM_ENERGY);
        place_resources(&mut grid, Cell::SciencePOI, NUM_SCIENCE_POI);

        Map {
            grid: Arc::new(Mutex::new(grid)),
            robots,
        }
    }

    pub fn display(&self) {
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

    pub fn update_position(&self, id: usize, new_pos: (usize, usize)) {
        let mut robots = self.robots.lock().unwrap();
        robots.insert(id, new_pos);
    }

    pub fn is_empty_or_walkable(&self, pos: (usize, usize)) -> bool {
        let grid = self.grid.lock().unwrap();
        if grid[pos.1][pos.0] == Cell::Obstacle {
            return false;
        }
        true
    }

    pub fn find_base_position(&self) -> Option<(usize, usize)> {
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

    pub fn get_cell(&self, pos: (usize, usize)) -> Cell {
        let grid = self.grid.lock().unwrap();
        grid[pos.1][pos.0]
    }

    pub fn pick_up_resource(&self, pos: (usize, usize)) -> Option<Cell> {
        let mut grid = self.grid.lock().unwrap();
        match grid[pos.1][pos.0] {
            Cell::Mineral | Cell::Energy | Cell::SciencePOI => {
                let resource = grid[pos.1][pos.0];
                grid[pos.1][pos.0] = Cell::Empty;
                Some(resource)
            }
            _ => None,
        }
    }
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
    print!("{}[H", 27 as char);
}
