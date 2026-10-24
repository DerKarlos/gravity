//! Simmulation of one scene: create and move masses and the ship
//! and values, not given by the masses.

use crate::canvas::*;
use crate::masses::*;
use crate::ship::*;

pub const INTENDED_FPS: f64 = 50.; // frames per (ui) secound
pub const INTENDED_FRAME_TIME: f64 = 1. / INTENDED_FPS;

// ------------------- MASSES STRUCT/CLASS -------------------

pub struct Simulation {
    // todo: no pub!!!!
    scene: i16,
    text: String,
    pub ui_time_per_orbit: f64,
    pub simulated_seconds: f64,
    pub simulated_seconds_per_step: f64, // rename all step to frame ? No, a frame may execute multible frames
    run_mode: bool,
    follow: usize,
    frame_seconds_left: f64,
    iterations: f64, // iteration steps per render frame
}

impl Simulation {
    pub fn new(scene: i16) -> Simulation {
        Simulation {
            scene,
            text: String::new(),
            ui_time_per_orbit: 10., // The slowest orbit takes X seconds in the UI. Default, may be changed by the scene
            simulated_seconds: 0.0,
            simulated_seconds_per_step: 0.0,
            run_mode: true,
            follow: 0,
            frame_seconds_left: 0.,
            iterations: 1.0,
        }
    }

    pub fn scene(&self) -> i16 {
        self.scene
    }
    pub fn run_mode(&self) -> bool {
        self.run_mode
    }
    pub fn set_run_mode(&mut self, val: bool) {
        self.run_mode = val;
    }
    pub fn set_follow(&mut self, val: usize) {
        self.follow = val;
    }
    pub fn add_iterations(&mut self, val: f64) {
        self.iterations += val;
        self.iterations = self.iterations.clamp(0.01, 2000.);
        //println!("add_speed: {}", self.speed);
    }

    pub fn reduce_iterations(&mut self) {
        if self.iterations > 50. {
            self.iterations /= 2.;
        }
        // println!("iterations: {}", self.iterations);
    }

    pub fn set_seconds_per_orbit(&mut self, val: f64) {
        self.ui_time_per_orbit = val;
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn calc_orbit_seconds(&mut self, masses: &Masses) {
        // All masses are there, calculate the simulation seconds by the maximal ui-orbit-time
        self.simulated_seconds_per_step =
            masses.maximal_orbit_seconds() / INTENDED_FPS / self.ui_time_per_orbit;
    }

    pub fn iterate_one_step(&mut self, masses: &mut Masses) {
        masses.drag_and_move(self.simulated_seconds_per_step);
        masses.inc_position();
        self.simulated_seconds += self.simulated_seconds_per_step;
    }

    pub fn toggle_run_mode(&mut self) {
        self.run_mode = !self.run_mode;
    }

    pub fn draw(&mut self, masses: &Masses, ship: &Ship) {
        if self.follow > 0 {
            let position_of_followed_mass = masses.get_position_draw(self.follow);
            draw_set_view_xy(position_of_followed_mass * -1.);
        }
        draw_hud(&self.text, ship.speed());
    }

    // SIMULATE / ITERATE //
    // Simulate / iterate to the next position to be drawn in the next loop
    pub fn iterate(&mut self, masses: &mut Masses, ship: &mut Ship, frame_seconds: f64) {
        self.frame_seconds_left += frame_seconds;
        while self.frame_seconds_left > INTENDED_FRAME_TIME {
            self.frame_seconds_left -= INTENDED_FRAME_TIME / self.iterations;

            if self.run_mode() {
                ship.move_one_step(&self, masses);
                // also sets index to next step!:
                self.iterate_one_step(masses);
            }

            // Predict ship with new masses-posiitons-index or new burn-values etc.
            ship.predict_positions(&self, &masses);
        }
    }
}
