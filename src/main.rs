mod canvas;
mod controls;
mod masses;
mod scene;
mod ship;
mod simulation;
mod vec_space;

use crate::masses::*;
use crate::ship::*;
use canvas::*;
use controls::*;
use macroquad::prelude::*;
use scene::*;
use simulation::*;

pub fn conf() -> Conf {
    Conf {
        window_title: String::from("Gravity Sim Game"),
        window_width: 1000,
        window_height: 680,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // let (mut simulation, mut masses, mut ship, mut canvas) = set_scene(0);

    let mut masses = Masses::new();
    let mut ship = Ship::default();
    let mut canvas = Canvas::new(&conf());
    let mut simulation = set_scene(0, &mut canvas, &mut masses, &mut ship);

    let mut controls = Controls::new(&ship);

    let mut frame_delta_sum = 0.0;

    loop {
        if let Some(char) = get_char_pressed() {
            // println!("pressed char {:?}!", char);
            match char {
                '\u{1b}' => break, // KeyCode::Escape
                '\r' => {
                    // KeyCode::Enter
                    simulation.toggle_run_mode();
                    println!(
                        "planing_mode: {} {}",
                        simulation.run_mode, simulation.simulated_world_seconds
                    );
                }

                'r' => {
                    simulation = set_scene(simulation.scene, &mut canvas, &mut masses, &mut ship);
                }
                '0' => {
                    simulation = set_scene(0, &mut canvas, &mut masses, &mut ship);
                }
                '1' => {
                    simulation = set_scene(1, &mut canvas, &mut masses, &mut ship);
                }
                '2' => {
                    simulation = set_scene(2, &mut canvas, &mut masses, &mut ship);
                }
                '3' => {
                    simulation = set_scene(3, &mut canvas, &mut masses, &mut ship);
                }
                '4' => {
                    simulation = set_scene(4, &mut canvas, &mut masses, &mut ship);
                }
                '5' => {
                    simulation = set_scene(5, &mut canvas, &mut masses, &mut ship);
                }

                _ => (), // println!("Char not used: {:?}!", char),
            }
        }

        clear_background(BLACK);

        canvas.draw_grid();
        simulation.draw_text(&masses, &canvas);
        masses.draw(&canvas); // incl. prediction
        ship.draw(&canvas);

        // simulate next position to be drawn in the next loop
        let frame_delta_time: f64 = (get_frame_time() as f64).min(1.0);
        frame_delta_sum += frame_delta_time;

        // Simulate nothing or one ore some simulation steps
        controls.key_down(&mut ship, &mut canvas, frame_delta_time);

        while frame_delta_sum > SIMULATION_STEP_TIME {
            frame_delta_sum -= SIMULATION_STEP_TIME;

            if simulation.run_mode {
                ship.move_one_step(&simulation, &masses);
                // also sets index to next step!
                simulation.simulate_one_step(&mut masses);

                //ship.
            }

            // Predict ship with new masses index or new burn values etc.
            ship.predict_positions(&simulation, &masses);
        }

        next_frame().await
    }
}
