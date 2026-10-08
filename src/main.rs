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
use controls::*;

use macroquad::prelude::{BLACK, clear_background, get_char_pressed, get_frame_time, next_frame};
use scene::*;
use simulation::*;

const SCENE: i16 = 7;

#[macroquad::main("Gravity Sim Experience")]
async fn main() {
    // let (mut simulation, mut masses, mut ship, mut canvas) = set_scene(0);

    let (mut simulation, mut masses, mut ship) = set_scene(SCENE);

    let mut controls = Controls::new(&ship);

    let mut frame_seconds_left = 0.0;

    loop {
        if let Some(char) = get_char_pressed() {
            // println!("pressed char {:?}!", char);
            match char {
                '\u{1b}' => break, // KeyCode::Escape
                '\r' => {
                    // KeyCode::Enter
                    simulation.toggle_run_mode();
                    //println!(  "planing_mode: {} {}",simulation.run_mode, simulation.simulated_world_seconds );
                }

                '0'..='9' | 'r' => {
                    let id = if char == 'r' {
                        simulation.scene()
                    } else {
                        char as i16 - 48
                    };
                    println!("scene = {}", id);
                    (simulation, masses, ship) = set_scene(id);
                    controls = Controls::new(&ship);
                }
                _ => (), // println!("Char not used: {:?}!", char),
            }
        }

        clear_background(BLACK);

        draw_grid();
        simulation.draw_text(&masses);
        masses.draw_masses(); // incl. prediction
        ship.draw(&masses);

        // simulate next position to be drawn in the next loop
        let frame_seconds: f64 = (get_frame_time() as f64).min(1.0);
        frame_seconds_left += frame_seconds;

        // Simulate nothing or one ore some simulation steps
        controls.key_down(&mut ship, &mut masses, frame_seconds);

        while frame_seconds_left > SIMULATION_STEP_SECONDS {
            frame_seconds_left -= SIMULATION_STEP_SECONDS;

            if simulation.run_mode() {
                ship.move_one_step(&simulation, &masses);
                // also sets index to next step!
                simulation.simulate_one_step(&mut masses);
            }

            // Predict ship with new masses index or new burn values etc.
            ship.predict_positions(&simulation, &masses);
        }

        next_frame().await
    }
}
