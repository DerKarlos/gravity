use crate::canvas::draw_set_view_angle;
use crate::masses::*;
use crate::simulation::*;
use crate::vec_space::*;

pub const A_BURN: f64 = 0.3;

pub struct Ship {
    mass: Mass,
    burn_start: f64,
    burn_seconds: f64,
    burn_acceleration: f64,
    position: VecSpace,
    velocity: VecSpace,
    _start_angle_in_orbit: f64,
    rotation_a: usize, // why is this done in ship??? move to masses?
    rotation_b: usize,
}

impl Ship {
    pub fn default() -> Ship {
        Ship {
            mass: Mass::zero(),
            burn_start: 0.0,
            burn_seconds: 0.0,
            burn_acceleration: A_BURN,
            position: VecSpace::ZERO,
            velocity: VecSpace::ZERO,
            _start_angle_in_orbit: 0.,
            rotation_a: 0,
            rotation_b: 0,
        }
    }

    pub fn get_burn_values(&self) -> (f64, f64, f64) {
        (self.burn_start, self.burn_seconds, self.burn_acceleration)
    }

    pub fn speed(&self) -> f64 {
        self.velocity.length()
    }

    pub fn set_rotation(&mut self, start: f64, a: usize, b: usize) {
        self._start_angle_in_orbit = start;
        self.rotation_a = a;
        self.rotation_b = b;
    }

    pub fn set_burn(&mut self, start: f64, seconds: f64) {
        self.burn_start = start + 0.01;
        self.burn_seconds = seconds;
    }

    pub fn set_in_orbit(&mut self, masses: &mut Masses, data: &MassData, orbits: usize) {
        self.mass = masses.set_in_orbit(data, orbits);
    }

    pub fn move_one_step(&mut self, simulation: &Simulation, masses: &Masses) {
        let acceleration_vector =
            masses.drag_at_position(self.mass.position(), masses.positions_index());
        self.mass.ship_accelerate_vec(acceleration_vector);

        self.burn(simulation, simulation.simulated_seconds);

        self.mass
            .move_seconds(simulation.simulated_seconds_per_step, 0); // ship moves always at index 0

        let rotate = self.rotation_a != self.rotation_b;
        if rotate {
            let position_a = masses.get_position(self.rotation_a);
            let position_b = masses.get_position(self.rotation_b);
            let angle = (position_b - position_a).to_angle().to_degrees();
            //println!("angle: {}", angle);
            draw_set_view_angle(-angle);
        }
    }

    fn burn(&mut self, simulation: &Simulation, seconds: f64) {
        let start = self.burn_start;
        let end = self.burn_start + self.burn_seconds;

        //println!("burn: {} / {}", start, seconds);
        if seconds > start && seconds < end {
            let mut burn = self.burn_acceleration;
            let delta = seconds - start;
            if delta < simulation.simulated_seconds_per_step {
                burn = burn / simulation.simulated_seconds_per_step * delta;
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
        self.position = self.mass.position();
        self.velocity = self.mass.velocity();

        let mut seconds = simulation.simulated_seconds;
        let mut drag_index = masses.positions_index();
        for move_index in 1..PREDICT_COUNT {
            // lett all masses drag the ship
            let acceleration_vector = masses.drag_at_position(self.mass.position(), drag_index);
            self.mass.ship_accelerate_vec(acceleration_vector);

            self.burn(simulation, seconds);

            self.mass
                .move_seconds(simulation.simulated_seconds_per_step, move_index);

            drag_index += 1;
            drag_index %= PREDICT_COUNT;
            seconds += simulation.simulated_seconds_per_step;
        }

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
