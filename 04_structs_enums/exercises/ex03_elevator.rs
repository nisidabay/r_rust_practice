// Exercise: Elevator with floor tracking using an enum.
// ElevatorState has Moving { target: i32 } and Idle { floor: i32 } variants.

enum ElevatorState {
    Moving { target: i32 },
    Idle { floor: i32 },
}

impl ElevatorState {
    fn current_floor(&self) -> i32 {
        match self {
            ElevatorState::Moving { target } => *target,
            ElevatorState::Idle { floor } => *floor,
        }
    }

    fn description(&self) -> String {
        match self {
            ElevatorState::Moving { target } => format!("Moving to floor {target}"),
            ElevatorState::Idle { floor } => format!("Idle at floor {floor}"),
        }
    }
}

fn main() {
    let idle = ElevatorState::Idle { floor: 3 };
    assert_eq!(idle.current_floor(), 3);
    assert_eq!(idle.description(), "Idle at floor 3");

    let moving = ElevatorState::Moving { target: 7 };
    assert_eq!(moving.current_floor(), 7);
    assert_eq!(moving.description(), "Moving to floor 7");

    println!("All elevator tests passed!");
}
