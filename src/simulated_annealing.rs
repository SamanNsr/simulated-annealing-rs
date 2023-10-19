use rand::{Rng, RngCore};

use crate::problem::{Problem};

#[derive(Debug, Clone)]
pub struct SimulatedAnnealing {
    pub iterations: u32,
    pub initial_temperature: f32,
    pub temp_threshold: f32,
    pub alpha: f32,
}

impl SimulatedAnnealing {
    pub fn solve<P>(&self, rng: &mut dyn RngCore, individual: &P) -> P::State where P: Problem + ?Sized {
        let mut state = individual.initial_state();
        let mut energy = individual.energy(&state);
        let mut temperature = self.initial_temperature;
        let mut best_state = state.clone();

        for _ in 0 .. self.iterations {
                let next_state = individual.new_state(rng, &state);
                let new_energy = individual.energy(&next_state);

            state = {
                let de = new_energy - energy;
                println!("Energy: obj= {}", new_energy);
                if de < 0.0 || rng.gen::<f32>() <= std::f32::consts::E.powf(-de / temperature) {
                    energy = new_energy;
                    next_state
                } else {
                    state
                }
            };

            if individual.energy(&state)  > individual.energy(&best_state)  {
                best_state = state.clone();
            }

            temperature *= self.alpha;
            if self.temp_threshold >= temperature {
                break
            }
        }

        best_state
    }
}