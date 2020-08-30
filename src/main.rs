use rand::prelude::*;

mod elevator;

use crate::elevator::{Direction, DropoffRequest, Elevator, PickupRequest};

const ELEVATOR_COUNT: u16 = 3;
const SECS_BETWEEN_FLOORS: u16 = 5;
const DROPOFF_SECS: u16 = 10;
const PICKUP_SECS: u16 = 10;

fn main() {
    let mut elevator_bank = initialize_elevators();
    println!("We've got {} lovely elevators :)", elevator_bank.len());
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let ind = rng.gen::<usize>() % elevator_bank.len();
        elevator_bank[ind].add_pickup_request(PickupRequest { floor: 0 });
    }
}

fn initialize_elevators() -> Vec<Elevator> {
    let mut elevators: Vec<Elevator> = Vec::new();
    for i in 0..ELEVATOR_COUNT {
        let mut e = Elevator {
            current_floor: 0,
            current_direction: Direction::Stationary,
            dropoff_requests: vec![],
            pickup_requests: vec![],
        };
        elevators.push(e);
    }
    elevators
}
