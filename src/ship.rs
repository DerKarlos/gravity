use crate::masses::*;
use crate::simulation::*;
use crate::vec_space::*;

const A_BURN: f64 = 0.3;

pub struct Ship {
    pub mass: Mass,
    pub burn_start: f64,
    pub burn_seconds: f64,
    pub burn_acceleration: f64,
    position: VecSpace,
    velocity: VecSpace,
    _rotation_start: f64,
    rotation_a: usize,
    rotation_b: usize,
}

impl Ship {
    pub fn default() -> Ship {
        Ship {
            mass: Mass::_zero(),
            burn_start: 0.0,
            burn_seconds: 0.0,
            burn_acceleration: A_BURN,
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            _rotation_start: 0.,
            rotation_a: 0,
            rotation_b: 0,
        }
    }

    pub fn _set_rotation(&mut self, start: f64, a: usize, b: usize) {
        self._rotation_start = start;
        self.rotation_a = a;
        self.rotation_b = b;
    }

    pub fn set_burn(&mut self, start: f64, seconds: f64) {
        self.burn_start = start;
        self.burn_seconds = seconds;
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

        self.burn(simulation, simulation.simulated_seconds);

        self.mass
            .move_seconds(simulation.simulated_seconds_per_step, 0); // moves always at index 0
    }

    fn burn(&mut self, simulation: &Simulation, seconds: f64) {
        let start = self.burn_start;
        let end = self.burn_start + self.burn_seconds;

        println!("burn: {} / {}", start, seconds);
        if seconds > start && seconds < end {
            let mut burn = self.burn_acceleration;
            let delta = seconds - start;
            if delta < simulation.simulated_seconds_per_step {
                burn = burn / simulation.simulated_seconds_per_step * delta;
                //println!("burn: {} / {}", burn, self.burn_start);
            }
            self.mass.ship_accelerate_ahead(burn);
        }
    }

    // prediktor for ship: save, predict, restore
    // (the ship is moved independend of masses positions_index)
    pub fn predict_positions(&mut self, simulation: &Simulation, masses: &Masses) {
        let rotate = self.rotation_a == self.rotation_b;

        self.position = self.mass.get_position();
        self.velocity = self.mass.get_velocity();

        let mut seconds = simulation.simulated_seconds;
        let mut drag_index = masses.positions_index();
        for move_index in 1..PREDICT_COUNT {
            // lett all masses drag the ship
            let acceleration_vector = masses.drag_at_position(self.mass.get_position(), drag_index);
            self.mass.ship_accelerate_vec(acceleration_vector);

            self.burn(simulation, seconds);

            self.mass
                .move_seconds(simulation.simulated_seconds_per_step, move_index);

            if rotate {
                //let position_a = masses.get_position_from_index(self.rotation_a, move_index);
                //let position_b = masses.get_position_from_index(self.rotation_b, move_index);
                //let angle = position_a.angle_to(position_b);
            }

            drag_index += 1;
            drag_index %= PREDICT_COUNT;
            seconds += simulation.simulated_seconds_per_step;
        }

        self.mass.set_position(self.position);
        self.mass.set_velocity(self.velocity);
    }

    pub fn draw(&self, masses: &Masses) {
        self.mass
            .draw(0, masses.get_predict_show() as usize, masses.predict_count);
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
