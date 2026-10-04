//! Simmulation of one scene: create and move masses and the ship
//! and values, not given by the masses.

use crate::canvas::*;
use crate::masses::*;
use macroquad::prelude::*;

// About like the framerate in Hz, but will be checked and repeated if needed
pub const SIMULATION_STEPS_PER_APP_SECOND: f64 = 50.;
pub const SIMULATION_STEP_TIME: f64 = 1. / SIMULATION_STEPS_PER_APP_SECOND;

// ------------------- MASSES STRUCT/CLASS -------------------

pub struct Simulation {
    // todo: no pub!!!!
    pub scene: i16,
    text: String,
    pub app_seconds_per_orbit: f64,
    pub simulated_world_seconds: f64,
    pub world_seconds_per_step: f64, // rename all step to frame ???
    pub run_mode: bool,
}

impl Simulation {
    pub fn new(scene: i16) -> Simulation {
        Simulation {
            scene,
            text: String::new(),
            app_seconds_per_orbit: 10., // default, may be changed by the scene
            simulated_world_seconds: 0.0,
            world_seconds_per_step: 0.0,
            run_mode: true,
        }
    }

    pub fn set_seconds_per_orbit(&mut self, val: f64) {
        self.app_seconds_per_orbit = val;
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn app_to_world_seconds(&self, time: f64) -> f64 {
        let world_seconds_per_app_second =
            self.world_seconds_per_step * SIMULATION_STEPS_PER_APP_SECOND;
        time * world_seconds_per_app_second
    }

    pub fn set_orbit_time(&mut self, masses: &Masses) {
        // All masses are there, calculate the simulation time by the maximal orbit time
        self.world_seconds_per_step = masses.maximal_orbit_time()
            / SIMULATION_STEPS_PER_APP_SECOND
            / self.app_seconds_per_orbit;
    }

    // initially simulate all the future positinos

    pub fn simulate_one_step(&mut self, masses: &mut Masses) {
        masses.drag_and_move(self.world_seconds_per_step);
        masses.inc_position();
        self.simulated_world_seconds += self.world_seconds_per_step;
    }

    pub fn toggle_run_mode(&mut self) {
        self.run_mode = !self.run_mode;
        if !self.run_mode {
            // ???
            //let x = 1e4;
            // let y = 1e3;
            // ??? self.start_time = self.simulated_seconds + y * 2.;
            // ??? self.burn_time = y;
        }
    }

    pub fn draw_text(&mut self, masses: &Masses) {
        draw_hud(&self.text, masses.get_position());
    }
}
