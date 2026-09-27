mod canvas;
mod controls;
mod masses;
mod scene;
mod ship;
mod simulation;
mod vec_space;

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

    let (mut simulation, mut masses, mut ship, mut canvas) = set_scene(0);

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
                    (simulation, masses, ship, canvas) = set_scene(simulation.scene);
                }
                '0' => {
                    (simulation, masses, ship, canvas) = set_scene(0);
                }
                '1' => {
                    (simulation, masses, ship, canvas) = set_scene(1);
                }
                '2' => {
                    (simulation, masses, ship, canvas) = set_scene(2);
                }
                '3' => {
                    (simulation, masses, ship, canvas) = set_scene(3);
                }
                '4' => {
                    (simulation, masses, ship, canvas) = set_scene(4);
                }
                '5' => {
                    (simulation, masses, ship, canvas) = set_scene(5);
                }

                _ => (), // println!("Char not used: {:?}!", char),
            }
        }

        clear_background(BLACK);

        canvas.draw(); // grid
        simulation.draw(&masses, &canvas); // text
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
