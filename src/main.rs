mod canvas;
mod masses;
mod scene;
mod ship;
mod simulation;
mod vec_space;

use canvas::*;
use macroquad::prelude::*;
use scene::*;
use ship::*;
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

    let (mut simulation, mut masses, mut ship, mut canvas) = set_scene(5);

    let mut frame_delta_sum = 0.0;

    loop {
        if let Some(char) = get_char_pressed() {
            // println!("pressed char {:?}!", char);
            match char {
                '\u{1b}' => break, // KeyCode::Escape
                '\r' => {
                    // KeyCode::Enter
                    simulation.toggle_planing_mode();
                    println!(
                        "planing_mode: {} {}",
                        simulation.run_mode, simulation.simulated_seconds
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
        key_down(&mut ship, &mut canvas, SIMULATION_STEP_TIME);

        while frame_delta_sum > SIMULATION_STEP_TIME {
            frame_delta_sum -= SIMULATION_STEP_TIME;

            if simulation.run_mode {
                ship.move_0(&simulation, &masses);
                // also sets index to next step!
                simulation.simulate_one_step(&mut masses);

                //ship.
            }

            // Predict ship with new masses index
            ship.predict_positions(&simulation, &masses);
        }

        next_frame().await
    }
}

fn key_down(ship: &mut Ship, canvas: &mut Canvas, _simulation_step_time: f64) {
    if is_key_down(KeyCode::Space) {
        //masses.ship_accelerate(simulation_step_time);
    }
    if is_key_down(KeyCode::Backspace) {
        //masses.ship_accelerate(-simulation_step_time)
    }
    if is_key_down(KeyCode::Right) {
        ship.planing_start_time(1.);
    }
    if is_key_down(KeyCode::Left) {
        ship.planing_start_time(-1.);
    }
    if is_key_down(KeyCode::Up) {
        ship.planing_burn_time(1.);
    }
    if is_key_down(KeyCode::Down) {
        ship.planing_burn_time(-1.);
    }

    if is_key_down(KeyCode::U) {
        canvas.mul_z_view(1.001);
    }
    if is_key_down(KeyCode::J) {
        canvas.mul_z_view(0.999);
    }
}
