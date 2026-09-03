use crate::canvas::*;
use crate::masses::*;
use macroquad::prelude::*;

/// Simmulation of one scenario: create and move masses and the ship
/// and values, not given by the masses.

// About like the framerate in Hz, but will be checked and repeated if needed
pub const SIMULATION_STEPS_PER_SECOND: f64 = 50.;
pub const SIMULATION_STEP_TIME: f64 = 1. / SIMULATION_STEPS_PER_SECOND;

// ------------------- MASSES STRUCT/CLASS -------------------

pub struct Simulation {
    // todo: no pub!!!!
    text: String,
    pub case: i16,
    pub masses: Masses,
    //ship: Mass,
    //ship_position: VecSpace,
    //ship_velocity: VecSpace,
    //pub positions_draw_and_write_index: usize,
    seconds_per_orbit: f64,
    // simulated, not UI time.
    pub simulated_seconds: f64,
    simulated_seconds_per_step: f64,
    pub run_mode: bool,
    pub start_time: f64,
    pub burn_time: f64,
}

impl Simulation {
    pub fn new(case: i16) -> Simulation {
        Simulation {
            text: String::new(),
            case,
            masses: Masses::new(),
            //ship: Mass::zero(),
            //ship_position: VecSpace::ZERO,
            //ship_velocity: VecSpace::ZERO,
            // draw oldest position, write new position, increment after write
            // positions_draw_and_write_index: 0,
            seconds_per_orbit: 10., // default, may be changed by the scenario
            simulated_seconds: 0.0,
            simulated_seconds_per_step: 0.0,
            run_mode: true,
            start_time: 0.0,
            burn_time: 0.0,
        }
    }

    pub fn set_seconds_per_orbit(&mut self, val: f64) {
        self.seconds_per_orbit = val;
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    //pub fn add_ship_in_orbit(&mut self, data: &MassData, orbits: usize) {
    //    let orbits = &mut self.masses[orbits];
    //    let mass = Mass::new(data, Some(orbits));
    //    self.ship = mass;
    //}

    pub fn set_orbit_time(&mut self) {
        // All masses are there, calculate the simulation time by the maximal orbit time
        self.simulated_seconds_per_step =
            self.masses.maximal_orbit_time() / SIMULATION_STEPS_PER_SECOND / self.seconds_per_orbit;
    }

    // initially simulate all the future positinos
    pub fn predict_positions(&mut self) {
        // All masses are there, calculate the simulation time by the maximal orbit time
        self.simulated_seconds_per_step =
            self.masses.maximal_orbit_time() / SIMULATION_STEPS_PER_SECOND / self.seconds_per_orbit;

        for _ in 1..PREDICT_COUNT {
            self.masses.drag_and_move(self.simulated_seconds_per_step);
            self.masses.inc_position();
        }
    }

    //pub fn predict_ship_positions(&mut self) {
    //    // the ship is moved independend of positions_index
    //    // prediktor for ship: save predict restore
    //    self.ship_position = self.ship.position;
    //    self.ship_velocity = self.ship.velocity;
    //
    //    let save = self.simulated_seconds;
    //
    //    let mut drag_index = self.positions_draw_and_write_index;
    //    for move_index in 1..PREDICT_COUNT {
    //        // lett all masses drag the ship
    //        for mass in &self.masses {
    //            mass.drag_from_position(&mut self.ship, drag_index);
    //        }
    //
    //        // User secounds to simulated seconds
    //        let start =
    //            self.start_time * SIMULATION_STEPS_PER_SECOND * self.simulated_seconds_per_step;
    //        let end = (self.start_time + self.burn_time)
    //            * SIMULATION_STEPS_PER_SECOND
    //            * self.simulated_seconds_per_step;
    //        if self.simulated_seconds > start && self.simulated_seconds < end {
    //            self.ship.ship_accelerate(A_BURN);
    //        }
    //
    //        self.ship
    //            .move_seconds(self.simulated_seconds_per_step, move_index);
    //        drag_index += 1;
    //        drag_index %= PREDICT_COUNT;
    //        self.simulated_seconds += self.simulated_seconds_per_step;
    //    }
    //    self.simulated_seconds = save;
    //
    //    self.ship.position = self.ship_position;
    //    self.ship.velocity = self.ship_velocity;
    //}

    pub fn simulate_one_step(&mut self) {
        self.masses.drag_and_move(self.simulated_seconds_per_step);
        self.masses.inc_position();
    }

    pub fn __simulate_step(&mut self) {
        // First (ship drag and move and) all tragging then all (other) moves

        // The ship is moved independend of positions_index
        //for mass in &self.masses {
        //    mass.drag_from_position(&mut self.ship, self.positions_draw_and_write_index);
        //}

        self.simulated_seconds_per_step =
            self.masses.maximal_orbit_time() / SIMULATION_STEPS_PER_SECOND / self.seconds_per_orbit;

        // User secounds to simulated seconds
        let start = self.start_time * SIMULATION_STEPS_PER_SECOND * self.simulated_seconds_per_step;
        let end = (self.start_time + self.burn_time)
            * SIMULATION_STEPS_PER_SECOND
            * self.simulated_seconds_per_step;
        if self.simulated_seconds > start && self.simulated_seconds < end {
            //    self.ship.ship_accelerate(A_BURN);
        }

        // Move ship with actual index
        //self.ship.move_seconds(self.simulated_seconds_per_step, 0);

        // Move masses and increment index
        self.masses.drag_and_move(self.simulated_seconds_per_step);
        self.simulated_seconds += self.simulated_seconds_per_step;
    }

    //pub fn ship_accelerate(&mut self, acceleration: f64) {
    //    //self.ship.ship_accelerate(acceleration);
    //}

    pub fn toggle_planing_mode(&mut self) {
        self.run_mode = !self.run_mode;
        if !self.run_mode {
            // ???
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

    //pub fn simulate_masses_step(&mut self, simulated_seconds_per_step: f64) {
    //    // First drag, sedound move
    //
    //    // Each mass drags each other mass, except itselfes
    //    for i in 0..self.masses.len() {
    //        for j in (i + 1)..self.masses.len() {
    //            let (left, right) = self.masses.split_at_mut(j);
    //
    //            let a = &mut left[i];
    //            let b = &mut right[0];
    //            a.drag(b);
    //            b.drag(a);
    //        }
    //    }
    //
    //    // Move the masses at the head of the prediction
    //    for mass in &mut self.masses {
    //        mass.move_seconds(
    //            simulated_seconds_per_step,
    //            self.positions_draw_and_write_index,
    //        );
    //    }
    //}

    pub fn draw(&mut self, canvas: &Canvas) {
        canvas.draw_hud(&self.text, self.masses.get_position());
        self.masses.draw(&canvas);

        // self.ship.draw(&self, 0);
    }
}
