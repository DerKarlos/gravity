use crate::masses::*;
use crate::simulation::*;
use crate::vec_space::*;

pub const A_BURN: f64 = 0.3;

pub struct Ship {
    mass: Mass,
    position: VecSpace,
    velocity: VecSpace,
    burn_start: f64,
    burn_seconds: f64,
    burn_acceleration: f64,
    interpolate: i32,
}

impl Ship {
    pub fn default() -> Ship {
        Ship {
            mass: Mass::zero(),
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            burn_start: 0.0,
            burn_seconds: 0.0,
            burn_acceleration: A_BURN,
            interpolate: 1,
        }
    }

    pub fn burn_values(&self) -> (f64, f64, f64) {
        (self.burn_start, self.burn_seconds, self.burn_acceleration)
    }

    pub fn speed(&self, simulation: &Simulation, masses: &Masses) -> f64 {
        let pos_delta_per_predict_time = self.mass.delta_step(masses.predict_show() as usize);
        pos_delta_per_predict_time.length() / simulation.simulated_seconds_per_step() as f64
    }

    pub fn set_burn(&mut self, start: f64, seconds: f64) {
        self.burn_start = start + 0.01;
        self.burn_seconds = seconds;
    }

    pub fn interpolate(&self) -> i32 {
        self.interpolate
    }
    pub fn set_interpolate(&mut self, val: i32) {
        if val > 100 {
            panic!("interpolate>100");
        }
        self.interpolate = val.max(1);
        println!("interpolate: {}", self.interpolate);
    }

    pub fn set_in_orbit(&mut self, masses: &mut Masses, data: &MassData, orbits: usize) {
        self.mass = masses.set_in_orbit(data, orbits);
    }

    pub fn move_one_step(&mut self, simulation: &Simulation, masses: &Masses) {
        self.move_one_step_interpolate(
            simulation,
            masses,
            masses.positions_index(),
            0, // ship vector allways starts at 0
            simulation.simulated_seconds(),
        );
    }

    pub fn move_one_step_interpolate(
        &mut self,
        simulation: &Simulation,
        masses: &Masses,
        position_index: usize,
        vector_index: usize,
        simulated_seconds: f64,
    ) {
        let interpolate = self.interpolate();

        for interpolation_step in 0..interpolate {
            let interpolated_seconds = simulation.simulated_seconds_per_step() / interpolate as f64;
            // lett all masses drag the ship
            let acceleration_vector = masses.drag_at_position(
                self.mass.position(),
                position_index,
                interpolation_step as f64 / interpolate as f64,
            );
            self.mass.ship_accelerate_vec(acceleration_vector);
            self.burn(
                simulation.simulated_seconds_per_step(),
                simulated_seconds + interpolated_seconds,
            );

            self.mass.move_seconds(interpolated_seconds, vector_index);
        }
    }

    fn burn(&mut self, simulated_seconds_per_step: f64, simulated_seconds: f64) {
        let start = self.burn_start;
        let end = self.burn_start + self.burn_seconds;

        //println!("burn: {} / {}", start, seconds);
        if simulated_seconds > start && simulated_seconds < end {
            let mut burn = self.burn_acceleration;
            let delta = simulated_seconds - start;
            if delta < simulated_seconds_per_step {
                burn = burn / simulated_seconds_per_step * delta;
                //println!("burn: {} / {}", burn, self.burn_start);
            }
            self.mass.accelerate_ahead(burn);
        }
    }

    pub fn accelerate_ahead(&mut self, acceleration: f64) {
        self.mass.accelerate_ahead(acceleration);
    }

    // prediktor for ship: save, predict, restore
    // (the ship is moved independend of masses positions_index)
    pub fn predict_positions(&mut self, simulation: &Simulation, masses: &Masses) {
        // save
        self.position = self.mass.position();
        self.velocity = self.mass.velocity();

        let mut simulated_seconds = simulation.simulated_seconds();
        let mut position_index = masses.positions_index();
        for vector_index in 1..PREDICT_COUNT {
            self.move_one_step_interpolate(
                simulation,
                masses,
                position_index,
                vector_index,
                simulated_seconds,
            );

            position_index += 1;
            position_index %= PREDICT_COUNT;
            simulated_seconds += simulation.simulated_seconds_per_step();
        }

        // restore
        self.mass.set_position(self.position);
        self.mass.set_velocity(self.velocity);
    }

    pub fn draw(&self, masses: &Masses) {
        // The ship does not wrap the positions. Predictin always start at index 0
        masses.draw_position_0(&self.mass);
    }

    pub fn set_start_seconds(&mut self, set: f64) {
        self.burn_start = set;
    }

    pub fn set_burn_seconds(&mut self, set: f64) {
        self.burn_seconds = set;
    }

    pub fn set_burn_acceleration(&mut self, set: f64) {
        self.burn_acceleration = set;
    }
}
