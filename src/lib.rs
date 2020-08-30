enum PersonStatus {
    Waiting,
    OnElevator,
}
impl Default for PersonStatus {
    fn default() -> Self {
        PersonStatus::Waiting
    }
}
#[derive(Default)]
struct Person {
    floor: i32,
    start_time: i32,
    end_time: i32,
    start_floor: i32,
    destination_floor: i32,
    status: PersonStatus,
}
enum Direction {
    GoingUp,
    GoingDown,
    Stationary,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Stationary
    }
}
#[derive(Default)]
struct Elevator<'a> {
    people: Vec<&'a Person>,
    current_floor: i32,
    current_direction: Direction,
    steps_to_next: i32,
}

impl<'a> Elevator<'a> {
    pub fn add_person(&mut self, p: &'a Person) {
        self.people.push(p);
    }
    pub fn wait(&mut self, steps: i32) {
        self.current_direction = Direction::Stationary;
        self.steps_to_next += steps;
    }

    pub fn step(&mut self) {
        if self.steps_to_next == 0 {
            println!("action!");
        }
    }
}
#[cfg(test)]
mod elevator_tests {
    use super::*;
    #[test]
    fn add_people() {
        let mut e1: Elevator = Default::default();
        assert_eq!(e1.people.len(), 0);
        let p1: Person = Default::default();
        e1.add_person(&p1);
        assert_eq!(e1.people.len(), 1);
    }
}
