//! Simmulation of one scene: create and move masses and the ship
//! and values, not given by the masses.

use crate::canvas::*;
use crate::masses::*;
use crate::ship::*;

pub const INTENDED_FPS: f64 = 50.; // frames per (ui) secound
pub const INTENDED_FRAME_TIME: f64 = 1. / INTENDED_FPS;

pub struct Simulation {
    // todo: no pub!!!!
    scene: i16,
    text: String,
    ui_time_per_orbit: f64,
    simulated_seconds: f64,
    simulated_seconds_per_step: f64,
    run_mode: bool,
    frame_seconds_left: f64,
    iterations_per_frame: f64,
    pub masses: Masses, // todo: instance it here as sub-struct later
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
            frame_seconds_left: 0.,
            iterations_per_frame: 1.0,
            masses: Masses::new(),
        }
    }

    pub fn simulated_seconds(&self) -> f64 {
        self.simulated_seconds
    }

    pub fn simulated_seconds_per_step(&self) -> f64 {
        self.simulated_seconds_per_step
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
    pub fn add_iterations(&mut self, val: f64) {
        self.iterations_per_frame += val;
        self.iterations_per_frame = self.iterations_per_frame.clamp(0.01, 2000.);
        //println!("add_speed: {}", self.speed);
    }

    pub fn reduce_iterations(&mut self) {
        if self.iterations_per_frame > 50. {
            self.iterations_per_frame /= 2.;
        }
        // println!("iterations: {}", self.iterations);
    }

    pub fn set_seconds_per_orbit(&mut self, val: f64) {
        self.ui_time_per_orbit = val;
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn calc_orbit_seconds(&mut self) {
        // All masses are there, calculate the simulation seconds by the maximal ui-orbit-time
        self.simulated_seconds_per_step =
            self.masses.maximal_orbit_seconds() / INTENDED_FPS / self.ui_time_per_orbit;
    }

    pub fn iterate_one_step(&mut self) {
        self.masses.drag_and_move(self.simulated_seconds_per_step);
        self.simulated_seconds += self.simulated_seconds_per_step;
        self.masses.inc_position();
    }

    pub fn toggle_run_mode(&mut self) {
        self.run_mode = !self.run_mode;
    }

    pub fn draw(&mut self, ship: &Ship) {
        draw_start();
        draw_hud(&self.text, ship.speed(self));
        self.masses.draw(); // incl. prediction, follow, rotation
    }

    // SIMULATE / ITERATE //
    // Simulate / iterate to the next position to be drawn in the next loop
    pub fn iterate(&mut self, ship: &mut Ship, frame_time: f64) {
        if frame_time > INTENDED_FRAME_TIME {
            self.reduce_iterations();
            // println!("frame_time: {}", frame_time)
        }
        self.frame_seconds_left += frame_time;
        while self.frame_seconds_left > INTENDED_FRAME_TIME {
            self.frame_seconds_left -= INTENDED_FRAME_TIME / self.iterations_per_frame;

            if self.run_mode() {
                ship.move_one_step(&self);
                // also sets index to next step!:
                self.iterate_one_step();
            }

            // Predict ship with new masses-posiitons-index or new burn-values etc.
            ship.predict_positions(&self);
        }
    }
}
