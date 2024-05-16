use noise::{NoiseFn, Perlin, Seedable};
use rand::{thread_rng, Rng, seq::SliceRandom};
use std::{collections::HashMap, fmt::Display, sync::mpsc::Receiver};

use crate::{renderer::Renderer, Message, NB_ROBOTS};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Blank,
    Robot(u32),
    Obstacle,
    Base,
    Minerai,
    Energie,
}

impl Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            CellType::Blank => ' ',
            CellType::Robot(id) => match id {
                0 => '@',
                1 => '%',
                2 => '#',
                3 => '*',
                4 => '+',
                _ => unimplemented!(),
            },
            CellType::Obstacle => 'X',
            CellType::Base => 'B',
            CellType::Energie => 'E',
            CellType::Minerai => 'M',
        };
        write!(f, "{}", c)
    }
}

pub type Map2D = Vec<Vec<CellType>>;
pub type Position = (i32, i32);

pub const INITIAL_POSITION: Position = (10, 10);
pub const MAX_HEIGHT: i32 = 20;
pub const MAX_WIDTH: i32 = 20;
pub const MIN_HEIGHT: i32 = 0;
pub const MIN_WEIGHT: i32 = 0;
pub const NUM_MINERAI: usize = 5; 
pub const NUM_ENERGIE: usize = 5;

pub fn initialize_map() -> Map2D {
    let mut rng = thread_rng();
    let random_seed = rng.gen();

    let perlin = Perlin::new().set_seed(random_seed);

    let mut map = vec![vec![CellType::Blank; MAX_WIDTH as usize]; MAX_HEIGHT as usize];

    for y in 0..MAX_HEIGHT as usize {
        for x in 0..MAX_WIDTH as usize {
            if y == 0 || y == MAX_HEIGHT as usize - 1 || x == 0 || x == MAX_WIDTH as usize - 1 {
                map[y][x] = CellType::Obstacle;
            } else {
                let noise_value = perlin.get([x as f64 / 10.0, y as f64 / 10.0]);
                if noise_value > 0.5 {
                    map[y][x] = CellType::Obstacle;
                } else {
                    map[y][x] = CellType::Blank;
                }
            }
        }
    }
    let mut available_positions = Vec::new();
    for y in 1..(MAX_HEIGHT as usize - 1) {
        for x in 1..(MAX_WIDTH as usize - 1) {
            if map[y][x] == CellType::Blank {
                available_positions.push((x, y));
            }
        }
    }
    available_positions.shuffle(&mut rng);
    if let Some((x, y)) = available_positions.pop() {
        map[y][x] = CellType::Base;
    }
    for _ in 0..NUM_MINERAI {
        if let Some((x, y)) = available_positions.pop() {
            map[y][x] = CellType::Minerai;
        }
    }
    for _ in 0..NUM_ENERGIE {
        if let Some((x, y)) = available_positions.pop() {
            map[y][x] = CellType::Energie;
        }
    }
    map
}

pub fn clean_map(map: &mut Map2D) {
    map.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|c| *c = CellType::Blank));
}

pub fn initialize_positions() -> HashMap<u32, (i32, i32)> {
    let mut positions = HashMap::new();
    for id in 0..NB_ROBOTS {
        positions.insert(id, INITIAL_POSITION);
    }
    positions
}

pub fn update_and_draw_map(
    rx: &Receiver<Message>,
    positions: &mut HashMap<u32, Position>,
    map: &mut Map2D,
    renderer: &dyn Renderer,
) {
    if let Ok(Message::NewPosition { id, dx, dy }) = rx.recv() {
        let mut display_map = map.clone();
        update_positions_map(positions, map, &mut display_map, id, dx, dy);

        renderer.clean();
        renderer.draw_map(&display_map);
    }
}

pub fn update_positions_map(
    positions: &mut HashMap<u32, Position>,
    map: &mut Map2D,
    display_map: &mut Map2D,
    id: u32,
    dx: i32,
    dy: i32,
) {
    if let Some(position) = positions.get_mut(&id) {
        position.0 = (position.0 + dx).clamp(MIN_WEIGHT, MAX_WIDTH - 1);
        position.1 = (position.1 + dy).clamp(MIN_HEIGHT, MAX_HEIGHT - 1);
    }

    for (id, position) in positions {
        display_map[position.0 as usize][position.1 as usize] = CellType::Robot(*id);
    }
}
