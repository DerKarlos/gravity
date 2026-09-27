use crate::canvas::*;
use crate::masses::*;
use crate::simulation::*;
use crate::vec_space::*;

const A_BURN: f64 = 0.3;

pub struct Ship {
    pub mass: Mass,
    pub burn_start: f64, // in secounds
    pub burn_time: f64,  // in secounds
    position: VecSpace,
    velocity: VecSpace,
    rotation_start: f64,
    rotation_a: usize,
    rotation_b: usize,
}

impl Ship {
    pub fn default() -> Ship {
        Ship {
            mass: Mass::_zero(),
            burn_start: 0.0,
            burn_time: 0.0,
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            rotation_start: 0.,
            rotation_a: 0,
            rotation_b: 0,
        }
    }

    pub fn reset(&mut self) {
        self.mass = Mass::_zero();
        self.burn_start = 0.0;
        self.burn_time = 0.0;
        self.position = VecSpace::ZERO;
        self.velocity = VecSpace::ZERO;
        self.rotation_start = 0.;
        self.rotation_a = 0;
        self.rotation_b = 0;
    }

    pub fn set_rotation(&mut self, start: f64, a: usize, b: usize) {
        self.rotation_start = start;
        self.rotation_a = a;
        self.rotation_b = b;
    }

    pub fn set_burn(&mut self, start: f64, time: f64) {
        self.burn_start = start; // s
        self.burn_time = time; //   s
    }

    pub fn set_in_orbit(&mut self, masses: &mut Masses, data: &MassData, orbits: usize) {
        let orbits = masses.get_from_index(orbits);
        let mass = Mass::new(data, Some(orbits));
        self.mass = mass;
    }

    pub fn move_one_step(&mut self, simulation: &Simulation, masses: &Masses) {
        let acceleration_vector =
            masses.drag_at_position(self.mass.get_position(), masses.positions_index());
        self.mass.ship_accelerate_vec(acceleration_vector);

        self.burn(simulation, simulation.simulated_world_seconds);

        self.mass.move_seconds(simulation.world_seconds_per_step, 0); // moves always at index 0
    }

    fn burn(&mut self, simulation: &Simulation, seconds: f64) {
        let start = simulation.app_to_world_seconds(self.burn_start);
        let end = simulation.app_to_world_seconds(self.burn_start + self.burn_time);

        if seconds > start && seconds < end {
            let mut burn = A_BURN;
            let delta = seconds - start;
            if delta < simulation.world_seconds_per_step {
                burn = burn / simulation.world_seconds_per_step * delta;
                //println!("burn: {} / {}", burn, self.burn_start);
            }
            self.mass.ship_accelerate_ahead(burn);
        }
    }

    // prediktor for ship: save, predict, restore
    // (the ship is moved independend of masses positions_index)
    pub fn predict_positions(&mut self, simulation: &Simulation, masses: &Masses) {
        //self.mass.move_seconds(
        //    simulation.simulated_seconds_per_step,
        //    masses.positions_index(),
        //);

        let rotate = self.rotation_a == self.rotation_b;

        self.position = self.mass.get_position();
        self.velocity = self.mass.get_velocity();

        let mut seconds = simulation.simulated_world_seconds;
        let mut drag_index = masses.positions_index();
        for move_index in 1..PREDICT_COUNT {
            // lett all masses drag the ship
            let acceleration_vector = masses.drag_at_position(self.mass.get_position(), drag_index);
            self.mass.ship_accelerate_vec(acceleration_vector);

            self.burn(simulation, seconds);

            self.mass
                .move_seconds(simulation.world_seconds_per_step, move_index);

            if rotate {
                //let position_a = masses.get_position_from_index(self.rotation_a, move_index);
                //let position_b = masses.get_position_from_index(self.rotation_b, move_index);
                //let angle = position_a.angle_to(position_b);
            }

            drag_index += 1;
            drag_index %= PREDICT_COUNT;
            seconds += simulation.world_seconds_per_step;
        }

        self.mass.set_position(self.position);
        self.mass.set_velocity(self.velocity);
    }

    pub fn draw(&self, canvas: &Canvas) {
        self.mass.draw(canvas, 0);
    }

    pub fn _planing_start_time(&mut self, set: f64) {
        self.burn_start += set * 0.001;
        println!("start_time {}", self.burn_start);
    }

    pub fn set_start_time(&mut self, set: f64) {
        self.burn_start = set;
    }

    pub fn set_burn_time(&mut self, set: f64) {
        self.burn_time = set;
    }

    pub fn _planing_burn_time(&mut self, set: f64) {
        self.burn_time *= 1. + set * 0.003;
        println!("burn_time {}", self.burn_time);
    }
}
