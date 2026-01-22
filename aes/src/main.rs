mod state;
use crate::state::State;

fn main() {
    let mut state = State::zero();

    state[(0, 0)] = 12;
    state[(3, 3)] = 13;
    println!("{}", state);
}
