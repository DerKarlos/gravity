use crate::mass::*;
use crate::vec_space::*;
use macroquad::prelude::*;

// ------------------- MASSES STRUCT/CLASS -------------------

pub struct Simulation {
    text: String,
    pub case: i16,
    masses: Vec<Mass>,
    ship: Mass,
    ship_position: VecSpace,
    ship_velocity: VecSpace,
    pub z_view: f64,
    pub z_grid: f64,
    pub positions_draw_and_write_index: usize,
    pub maximal_orbit_radius: f64,
    pub maximal_orbit_time: f64,
    pub seconds_per_orbit: f64,
    pub simulated_seconds: f64,
    pub simulated_seconds_per_step: f64,
    pub planing_mode: bool,
    pub start_time: f64,
    pub burn_time: f64,
}

impl Simulation {
    pub fn new(case: i16) -> Simulation {
        Simulation {
            text: String::new(),
            case,
            masses: Vec::new(),
            ship: Mass::zero(),
            ship_position: VecSpace::ZERO,
            ship_velocity: VecSpace::ZERO,
            z_view: 0.9,
            z_grid: 0.9,
            // draw oldest position, write new position, increment after write
            positions_draw_and_write_index: 0,
            maximal_orbit_radius: 1.0,
            maximal_orbit_time: 0.0,
            seconds_per_orbit: DEFAULT_SECONDS_PER_ORBIT,
            simulated_seconds: 0.0,
            simulated_seconds_per_step: 0.0,
            planing_mode: false,
            start_time: 0.0,
            burn_time: 0.0,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn add_mass_at_place(&mut self, data: &MassData) -> usize {
        let mass = Mass::new(data, None);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn add_mass_in_orbit(&mut self, data: &MassData, orbits: usize) -> usize {
        let orbits = &mut self.masses[orbits];
        self.maximal_orbit_radius = data.orbit_radius.max(self.maximal_orbit_radius);
        //println!("max orbit: {}", self.maximal_orbit);
        let mass = Mass::new(data, Some(orbits));
        self.maximal_orbit_time = self.maximal_orbit_time.max(mass.orbit_time);
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn add_ship_in_orbit(&mut self, data: &MassData, orbits: usize) {
        let orbits = &mut self.masses[orbits];
        let mass = Mass::new(data, Some(orbits));
        self.ship = mass;
    }

    // initially simulate all the future positinos
    pub fn predict_positions(&mut self) {
        // All masses are there, calculate the simulation time by the maximal orbit time
        self.simulated_seconds_per_step =
            self.maximal_orbit_time / SIMULATION_STEPS_PER_SECOND / self.seconds_per_orbit;

        for _ in 1..PREDICT_COUNT {
            self.simulate_masses_step(self.simulated_seconds_per_step);
            self.positions_draw_and_write_index = self.next_position(); //äää
        }
    }

    pub fn predict_ship_positions(&mut self) {
        // the ship is moved independend of positions_index
        // prediktor for ship: save predict restore
        self.ship_position = self.ship.position;
        self.ship_velocity = self.ship.velocity;

        let save = self.simulated_seconds;

        let mut drag_index = self.positions_draw_and_write_index;
        for move_index in 1..PREDICT_COUNT {
            // lett all masses drag the ship
            for mass in &self.masses {
                mass.drag_from_position(&mut self.ship, drag_index);
            }

            // User secounds to simulated seconds
            let start =
                self.start_time * SIMULATION_STEPS_PER_SECOND * self.simulated_seconds_per_step;
            let end = (self.start_time + self.burn_time)
                * SIMULATION_STEPS_PER_SECOND
                * self.simulated_seconds_per_step;
            if self.simulated_seconds > start && self.simulated_seconds < end {
                self.ship.ship_accelerate(A_BURN);
            }

            self.ship
                .move_seconds(self.simulated_seconds_per_step, move_index);
            drag_index += 1;
            drag_index %= PREDICT_COUNT;
            self.simulated_seconds += self.simulated_seconds_per_step;
        }
        self.simulated_seconds = save;

        self.ship.position = self.ship_position;
        self.ship.velocity = self.ship_velocity;
    }

    pub fn simulate_step(&mut self) {
        // First (ship drag and move and) all tragging then all (other) moves

        // The ship is moved independend of positions_index
        for mass in &self.masses {
            mass.drag_from_position(&mut self.ship, self.positions_draw_and_write_index);
        }

        self.simulated_seconds_per_step =
            self.maximal_orbit_time / SIMULATION_STEPS_PER_SECOND / self.seconds_per_orbit;

        // User secounds to simulated seconds
        let start = self.start_time * SIMULATION_STEPS_PER_SECOND * self.simulated_seconds_per_step;
        let end = (self.start_time + self.burn_time)
            * SIMULATION_STEPS_PER_SECOND
            * self.simulated_seconds_per_step;
        if self.simulated_seconds > start && self.simulated_seconds < end {
            self.ship.ship_accelerate(A_BURN);
        }

        // Move ship with actual index
        self.ship.move_seconds(self.simulated_seconds_per_step, 0);

        // Move masses and increment index
        self.simulate_masses_step(self.simulated_seconds_per_step);
        self.simulated_seconds += self.simulated_seconds_per_step;
    }

    pub fn ship_accelerate(&mut self, acceleration: f64) {
        self.ship.ship_accelerate(acceleration);
    }

    pub fn toggle_planing_mode(&mut self) {
        self.planing_mode = !self.planing_mode;
        if self.planing_mode {
            //let x = 1e4;
            // let y = 1e3;
            // ??? self.start_time = self.simulated_seconds + y * 2.;
            // ??? self.burn_time = y;
        }
    }

    pub fn planing_start_time(&mut self, set: f64) {
        self.start_time += set * 0.001;
        println!("start_time {}", self.start_time);
    }

    pub fn planing_burn_time(&mut self, set: f64) {
        self.burn_time *= 1. + set * 0.003;
        println!("burn_time {}", self.burn_time);
    }

    pub fn simulate_masses_step(&mut self, simulated_seconds_per_step: f64) {
        // First drag, sedound move

        // Each mass drags each other mass, except itselfes
        for i in 0..self.masses.len() {
            for j in (i + 1)..self.masses.len() {
                let (left, right) = self.masses.split_at_mut(j);

                let a = &mut left[i];
                let b = &mut right[0];
                a.drag(b);
                b.drag(a);
            }
        }

        // Move the masses at the head of the prediction
        for mass in &mut self.masses {
            mass.move_seconds(
                simulated_seconds_per_step,
                self.positions_draw_and_write_index,
            );
        }
    }

    pub fn next_position(&self) -> usize {
        (self.positions_draw_and_write_index + 1) % PREDICT_COUNT
    }

    pub fn scale(&self, position: &VecSpace) -> VecSpace {
        let window_center: VecSpace =
            VecSpace::new(WINDOW_WIDTH as f64 / 2., WINDOW_HEIGHT as f64 / 2.);
        // Scale by view, divide by scene multiply by screen, add screen center
        *position * (self.z_view / self.maximal_orbit_radius * MAX_PIXEL_FROM_CENTER as f64)
            + window_center
    }

    pub fn draw(&mut self) {
        draw_text(
            format!("{} {}", self.text, self.positions_draw_and_write_index).as_str(),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );

        for mass in &self.masses {
            mass.draw(&self, self.positions_draw_and_write_index);
        }

        self.ship.draw(&self, 0);
    }
}
