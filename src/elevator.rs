pub struct PickupRequest {
    pub floor: i16,
}

pub struct DropoffRequest {
    pub floor: i16,
}

pub enum Direction {
    GoingUp,
    GoingDown,
    Stationary,
}
pub struct Elevator {
    pub pickup_requests: Vec<PickupRequest>,
    pub dropoff_requests: Vec<DropoffRequest>,
    pub current_floor: i16,
    pub current_direction: Direction,
}

impl Elevator {
    pub fn add_pickup_request(&mut self, req: PickupRequest) {
        self.pickup_requests.push(req);
    }
}
