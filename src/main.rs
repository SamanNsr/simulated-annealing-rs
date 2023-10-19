use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::RngCore;
use crate::problem::{Problem};
use crate::simulated_annealing::SimulatedAnnealing;

mod simulated_annealing;
mod problem;

fn main()  {
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let sa =  SimulatedAnnealing {
        iterations: 1000000,
        initial_temperature: 100.0,
        temp_threshold: 10.0,
        alpha: 0.999,
    };

    let problem = OptimizationProblem {
        state_bounds: ((-3.0, 12.1), (4.1, 5.8)),
        max_step_size_x1: 0.3,
        max_step_size_x2: 0.1,
    };
    let (best_x1, best_x2) = sa.solve(&mut rng, &problem);
    println!("best: x1= {}, x2= {}, energy= {}", best_x1, best_x2, problem.energy( &(best_x1, best_x2)))

}

pub struct OptimizationProblem {
    pub state_bounds: ((f32, f32), (f32, f32)),
    pub max_step_size_x1: f32,
    pub max_step_size_x2: f32,
}

impl Problem for OptimizationProblem {
    type State = (f32, f32);
    fn initial_state(&self) -> Self::State {
        // You can choose an initial state, or just use random values
        (5.0, 4.0)
    }

    fn energy(&self, state: &Self::State) -> f32 {
        let (x1, x2) = *state;
        let result = 21.5 + x1 * f32::sin(4.0 * 3.14 * x1) + x2 * f32::sin(20.0 * 3.14 * x2);
        result
    }

    fn new_state(&self, rng: &mut dyn RngCore, state: &Self::State) -> Self::State {
        let ((min_x1, max_x1), (min_x2, max_x2)) = self.state_bounds;
        let (x1, x2) = *state;

        let new_x1 = {
            let perturbation = rng.gen::<f32>() * 2.0 - 1.0;
            let step_size = perturbation * self.max_step_size_x1;
            let candidate_x1 = x1 + step_size;
            candidate_x1.min(max_x1).max(min_x1) // Ensure the candidate value is within bounds
        };

        let new_x2 = {
            let perturbation = rng.gen::<f32>() * 2.0 - 1.0;
            let step_size = perturbation * self.max_step_size_x2;
            let candidate_x2 = x2 + step_size;
            candidate_x2.min(max_x2).max(min_x2) // Ensure the candidate value is within bounds
        };

        (new_x1, new_x2)
    }

}
