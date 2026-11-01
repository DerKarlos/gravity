//! Todo
//!
//! There are two "times" in this code:
//! - The simulated seconds. The symbol contains "seconds", may be "simulated_seconds"
//! - The time, the app is running. The symbol contains "time", may be "ui-time"

mod canvas;
mod controls;
mod mass;
mod masses;
mod scene;
mod ship;
mod simulation;
mod vec_space;

use controls::{Controls, KeyEvent};

use macroquad::prelude::{get_frame_time, next_frame};
use scene::{SCENE, set_scene};

#[macroquad::main("Gravity Sim Experience")]
async fn main() {
    let mut simulation = set_scene(SCENE);

    let mut controls = Controls::new(&simulation.ships[0]);

    loop {
        // FRAMES //
        let frame_time: f64 = (get_frame_time() as f64).min(1.0123);

        // INPUT //
        match controls.check_chars(&mut simulation) {
            KeyEvent::NONE => (),
            KeyEvent::BREAK => break,
            KeyEvent::SceneID(id) => {
                canvas::draw_reset();
                simulation = set_scene(id);
                controls = Controls::new(&simulation.ships[0]);
            }
        }
        controls.check_keys(&mut simulation, frame_time);

        // SIMULATE / ITERATE //
        simulation.iterate(frame_time);

        // DRAW //
        simulation.draw();

        next_frame().await
    }
}
