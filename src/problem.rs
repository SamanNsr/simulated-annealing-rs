use rand::RngCore;

pub trait Problem {
    type State: Clone;
    fn initial_state(&self) -> Self::State;
    fn energy(&self, state: &Self::State) -> f32;
    fn new_state(&self, rng: &mut dyn RngCore, state: &Self::State) -> Self::State;
}