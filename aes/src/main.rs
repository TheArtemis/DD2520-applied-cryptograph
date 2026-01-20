mod state;
use crate::state::State;

fn main() {
    println!("Hello, world!");

    let mut state = State::zero();
    state[(0, 0)] = 12;

    println!("{}", state[(0, 0)]);
}
