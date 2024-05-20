use std::sync::{Arc, Mutex};

use crate::{Cell, MAP_HEIGHT, MAP_WIDTH};

pub struct Base {
    pub position: (usize, usize),
    pub map: Arc<Mutex<Vec<Vec<Cell>>>>,
    pub collected_resources: Vec<Cell>,
}

impl Base {
    pub fn new(position: (usize, usize), map: Arc<Mutex<Vec<Vec<Cell>>>>) -> Self {
        Base {
            position,
            map,
            collected_resources: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource: Cell) {
        self.collected_resources.push(resource);
    }

    pub fn display_resources(&self) {
        let mut resource_count = [0; 3];
        for resource in &self.collected_resources {
            match resource {
                Cell::Mineral => resource_count[0] += 1,
                Cell::Energy => resource_count[1] += 1,
                Cell::SciencePOI => resource_count[2] += 1,
                _ => {}
            }
        }
        println!(
            "Base at position {:?} has collected the following resources:",
            self.position
        );
        println!("Minerals: {}", resource_count[0]);
        println!("Energy: {}", resource_count[1]);
        println!("Scientific Points of Interest: {}", resource_count[2]);
    }
}
