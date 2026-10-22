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

use canvas::draw_start;
use controls::{Controls, KeyEvent};

use macroquad::prelude::{get_frame_time, next_frame};
//use masses::*;
use scene::*;
use simulation::*;

//use crate::{canvas::draw_circle, vec_space::VecSpace};

#[macroquad::main("Gravity Sim Experience")]
async fn main() {
    // let (mut simulation, mut masses, mut ship, mut canvas) = set_scene(0);

    let (mut simulation, mut masses, mut ship) = set_scene(SCENE);

    let mut controls = Controls::new(&ship);

    loop {
        // INPUT //
        match controls.char_pressed(&mut simulation) {
            KeyEvent::NONE => (),
            KeyEvent::BREAK => break,
            KeyEvent::SceneID(id) => {
                canvas::draw_reset();
                (simulation, masses, ship) = set_scene(id);
                controls = Controls::new(&ship);
            }
        }
        let frame_seconds: f64 = (get_frame_time() as f64).min(1.0);
        if frame_seconds > SIMULATION_STEP_TIME {
            simulation.add_speed(-frame_seconds * 50.);
            println!("{}", frame_seconds)
        }
        controls.key_down(&mut simulation, &mut ship, &mut masses, frame_seconds);

        // SIMULATE //
        simulation.iterate(&mut masses, &mut ship, frame_seconds);

        // DRAW //
        draw_start();
        simulation.draw(&masses, &ship);
        masses.draw_masses(); // incl. prediction
        ship.draw(&masses); // incl. prediction

        next_frame().await
    }
}
