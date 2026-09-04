use crate::canvas::*;
use crate::masses::*;
use crate::simulation::*;
use crate::vec_space::*;

const A_BURN: f64 = 0.3;

pub struct Ship {
    pub mass: Mass,
    pub burn_start: f64,
    pub burn_time: f64,
    position: VecSpace,
    velocity: VecSpace,
}

impl Ship {
    pub fn default() -> Ship {
        Ship {
            mass: Mass::_zero(),
            burn_start: 0.0,
            burn_time: 0.0,
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
        }
    }

    pub fn set_burn(&mut self, start: f64, time: f64) {
        self.burn_start = start;
        self.burn_time = time;
    }

    pub fn set_in_orbit(&mut self, masses: &mut Masses, data: &MassData, orbits: usize) {
        let orbits = masses.get_from_index(orbits);
        let mass = Mass::new(data, Some(orbits));
        self.mass = mass;
    }

    pub fn move_0(&mut self, simulation: &Simulation, masses: &Masses) {
        let acceleration_vector =
            masses.drag_at_position(self.mass.position, masses.positions_index());
        self.mass.ship_accelerate_vec(acceleration_vector);

        // User secounds to simulated seconds
        let start = simulation.ui_to_sim_time(self.burn_start);
        let end = simulation.ui_to_sim_time(self.burn_start + self.burn_time);
        if simulation.simulated_seconds > start && simulation.simulated_seconds < end {
            self.mass.ship_accelerate_ahead(A_BURN);
        }

        self.mass
            .move_seconds(simulation.simulated_seconds_per_step, 0); // moves always at index 0
    }
    // prediktor for ship: save, predict, restore
    // (the ship is moved independend of masses positions_index)
    pub fn predict_positions(&mut self, simulation: &Simulation, masses: &Masses) {
        //self.mass.move_seconds(
        //    simulation.simulated_seconds_per_step,
        //    masses.positions_index(),
        //);

        self.position = self.mass.position;
        self.velocity = self.mass.velocity;

        let mut seconds = simulation.simulated_seconds;
        let mut drag_index = masses.positions_index();
        for move_index in 1..PREDICT_COUNT {
            // lett all masses drag the ship
            let acceleration_vector = masses.drag_at_position(self.mass.position, drag_index);
            self.mass.ship_accelerate_vec(acceleration_vector);

            // User secounds to simulated seconds
            let start = simulation.ui_to_sim_time(self.burn_start);
            let end = simulation.ui_to_sim_time(self.burn_start + self.burn_time);
            if seconds > start && seconds < end {
                self.mass.ship_accelerate_ahead(A_BURN);
            }

            self.mass
                .move_seconds(simulation.simulated_seconds_per_step, move_index);
            drag_index += 1;
            drag_index %= PREDICT_COUNT;
            seconds += simulation.simulated_seconds_per_step;
        }

        self.mass.position = self.position;
        self.mass.velocity = self.velocity;
    }

    pub fn draw(&self, canvas: &Canvas) {
        self.mass.draw(canvas, 0);
    }

    pub fn planing_start_time(&mut self, set: f64) {
        self.burn_start += set * 0.001;
        println!("start_time {}", self.burn_start);
    }

    pub fn planing_burn_time(&mut self, set: f64) {
        self.burn_time *= 1. + set * 0.003;
        println!("burn_time {}", self.burn_time);
    }
}
