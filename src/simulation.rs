use crate::canvas::*;
use crate::masses::*;
use macroquad::prelude::*;

/// Simmulation of one scene: create and move masses and the ship
/// and values, not given by the masses.

// About like the framerate in Hz, but will be checked and repeated if needed
pub const SIMULATION_STEPS_PER_SECOND: f64 = 50.;
pub const SIMULATION_STEP_TIME: f64 = 1. / SIMULATION_STEPS_PER_SECOND;

// ------------------- MASSES STRUCT/CLASS -------------------

pub struct Simulation {
    // todo: no pub!!!!
    pub scene: i16,
    text: String,
    pub seconds_per_orbit: f64,
    // simulated time, not the UI time!
    pub simulated_seconds: f64,
    pub simulated_seconds_per_step: f64,
    pub run_mode: bool,
}

impl Simulation {
    pub fn new(scene: i16) -> Simulation {
        Simulation {
            scene,
            text: String::new(),
            seconds_per_orbit: 10., // default, may be changed by the scene
            simulated_seconds: 0.0,
            simulated_seconds_per_step: 0.0,
            run_mode: true,
        }
    }

    pub fn set_seconds_per_orbit(&mut self, val: f64) {
        self.seconds_per_orbit = val;
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn ui_to_sim_time(&self, time: f64) -> f64 {
        time * SIMULATION_STEPS_PER_SECOND * self.simulated_seconds_per_step
    }

    pub fn set_orbit_time(&mut self, masses: &Masses) {
        // All masses are there, calculate the simulation time by the maximal orbit time
        self.simulated_seconds_per_step =
            masses.maximal_orbit_time() / SIMULATION_STEPS_PER_SECOND / self.seconds_per_orbit;
    }

    // initially simulate all the future positinos

    pub fn simulate_one_step(&mut self, masses: &mut Masses) {
        masses.drag_and_move(self.simulated_seconds_per_step);
        masses.inc_position();
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

    //pub fn drag_and_move <== simulate_masses_step(&mut self, simulated_seconds_per_step: f64) {
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

    pub fn draw(&mut self, masses: &Masses, canvas: &Canvas) {
        canvas.draw_hud(&self.text, masses.get_position());
    }
}
