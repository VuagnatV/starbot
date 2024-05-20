use actix::Actor;
use actix_rt::signal;
use std::sync::{Arc, Mutex};

use starbot::{Base, Cell, Map, Robot, Start, MAP_HEIGHT, MAP_WIDTH};

#[actix_rt::main]
async fn main() {
    let num_robots = 5;
    let map = Map::new();
    let base_map = Arc::new(Mutex::new(vec![vec![Cell::Unknown; MAP_WIDTH]; MAP_HEIGHT]));
    let base_position = map.find_base_position().unwrap();
    base_map.lock().unwrap()[base_position.1][base_position.0] = Cell::Base;
    let base = Arc::new(Mutex::new(Base::new(base_position, base_map.clone())));
    let mut robots = vec![];

    for id in 1..=num_robots {
        let robot = Robot::new(id, map.clone(), base_map.clone(), base.clone()).start();
        robot.do_send(Start);
        robots.push(robot);
    }

    actix_rt::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
    println!("Shutting down...");
}
