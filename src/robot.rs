use actix::prelude::*;
use rand::Rng;
use std::fmt::Write as FmtWrite;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{Base, Cell, Map, MAP_HEIGHT, MAP_WIDTH};

pub struct Robot {
    id: usize,
    position: (usize, usize),
    map: Map,
    personal_map: Vec<Vec<Cell>>,
    base_map: Arc<Mutex<Vec<Vec<Cell>>>>,
    base: Arc<Mutex<Base>>,
    carrying: Option<Cell>,
}

impl Robot {
    pub fn new(
        id: usize,
        map: Map,
        base_map: Arc<Mutex<Vec<Vec<Cell>>>>,
        base: Arc<Mutex<Base>>,
    ) -> Self {
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
            base,
            carrying: None,
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
                if self.personal_map[y][x] != Cell::Empty && base_map[y][x] == Cell::Empty {
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
                if base_map[y][x] != Cell::Empty && self.personal_map[y][x] == Cell::Empty {
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
        let base = self.base.lock().unwrap();
        base.display_resources();
    }
}

impl Actor for Robot {
    type Context = Context<Self>;
}

pub struct Start;

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

                if act.carrying.is_none() {
                    if let Some(resource) = act.map.pick_up_resource(new_position) {
                        act.carrying = Some(resource);
                    }
                }
            }

            if act.map.grid.lock().unwrap()[new_position.1][new_position.0] == Cell::Base {
                act.update_base_map();
                act.merge_maps();

                if let Some(resource) = act.carrying.take() {
                    let mut base = act.base.lock().unwrap();
                    base.add_resource(resource);
                }
            }

            act.update_personal_map(new_position);

            act.map.display();
            act.display_base_map();
        });
    }
}
