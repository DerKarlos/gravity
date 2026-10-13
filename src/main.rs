//! Todo
//!
//! There are two "times" in this code:
//! - The simulated seconds. The symbol contains "seconds", may be "simulated_seconds"
//! - The time, the app is running. The symbol contains "time", may be "ui-time"

mod canvas;
mod controls;
mod masses;
mod scene;
mod ship;
mod simulation;
mod vec_space;

use canvas::draw_grid;
use controls::{Controls, KeyEvent};

use macroquad::prelude::{BLACK, RED, clear_background, get_frame_time, next_frame};
use masses::*;
use scene::*;
use simulation::*;

use crate::{canvas::draw_circle, vec_space::VecSpace};

#[macroquad::main("Gravity Sim Experience")]
async fn main() {
    // let (mut simulation, mut masses, mut ship, mut canvas) = set_scene(0);

    let (mut simulation, mut masses, mut ship) = set_scene(SCENE);

    let mut controls = Controls::new(&ship);

    let mut frame_seconds_left = 0.0;

    loop {
        // INPUT //
        match controls.char_pressed(&mut simulation) {
            KeyEvent::NONE => (),
            KeyEvent::BREAK => break,
            KeyEvent::SceneID(id) => {
                (simulation, masses, ship) = set_scene(id);
                controls = Controls::new(&ship);
            }
        }
        let frame_seconds: f64 = (get_frame_time() as f64).min(1.0);
        controls.key_down(&mut ship, &mut masses, frame_seconds);

        // SIMULATE //
        //
        // Simulate next position to be drawn in the next loop
        frame_seconds_left += frame_seconds;

        // Simulate nothing or one ore some simulation steps
        while frame_seconds_left > SIMULATION_STEP_TIME {
            frame_seconds_left -= SIMULATION_STEP_TIME;

            if simulation.run_mode() {
                ship.move_one_step(&simulation, &masses);
                // also sets index to next step!
                simulation.simulate_one_step(&mut masses);
            }

            // Predict ship with new masses index or new burn values etc.
            ship.predict_positions(&simulation, &masses);
        }

        // DRAW //
        clear_background(BLACK);
        draw_grid();
        simulation.draw_text(&ship);
        masses.draw_masses(); // incl. prediction
        ship.draw(&masses); // incl. prediction

        let angle = masses.orbit_angle(1);
        let d: f64 = 30.;
        let r = d.to_radians();

        let x = VecSpace::new(0., km(370171.));
        let y = x.rotate_angle(angle - r);
        draw_circle(y, 22., RED);

        next_frame().await
    }
}
