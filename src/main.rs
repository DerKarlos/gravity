mod canvas;
mod masses;
mod ship;
mod simulation;
mod vec_space;

use canvas::*;
use macroquad::prelude::*;
use masses::*;
use ship::*;
use simulation::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Gravity Sim Game"),
        window_width: 1000,
        window_height: 680,
        window_resizable: false,
        ..Default::default()
    }
}

fn set_masses(
    scenario: i16,
    simulation: &mut Simulation,
    masses: &mut Masses,
    ship: &mut Ship,
    canvas: &mut Canvas,
) {
    // some masses
    let sun_data = MassData::fixstar("sun", YELLOW, km(1.3914e6), mass_sol(1.));
    let sun2_data = MassData::orbiter("sun2", GOLD, km(1.3914e6), mass_sol(1.), au(0.5));
    let earth_data = MassData::orbiter("earth", BLUE, km(12756.32), mass_earth(1.), au(1.));

    // more but 0.005 AE radius makes the orbit insable.
    let luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
    let _jupiter_d = MassData::orbiter("jupiter", GREEN, km(142984.0), kg(1.899e27), au(25e3));
    let comet_data = MassData::ellipse("comet", WHITE, km(500.0), kg(1e6), au(1.3), 0.4);
    let ship_data = MassData::orbiter("ship", WHITE, 10.0, 0.0, km(80000.)); // the real 300km are not visible

    match scenario {
        1 => {
            simulation.set_text("Sun, Earth");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&earth_data, sun);
        }

        2 => {
            simulation.set_text("double star");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&sun2_data, sun);
        }

        3 => {
            simulation.set_text("Earth & Luna & Ship");
            simulation.set_seconds_per_orbit(60.);
            let earth = masses.add_at_place(&earth_data);
            masses.add_in_orbit(&luna_data, earth);
            //simulation.masses.add_ship_in_orbit(&ship_data, earth);
        }

        4 => {
            simulation.set_text("Sun, Earth & Luna");
            let sun = masses.add_at_place(&sun_data);
            let earth = masses.add_in_orbit(&earth_data, sun);
            masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&comet_data, sun);
        }

        _ => {
            simulation.set_text("Test");
            let earth = masses.add_at_place(&earth_data);
            masses.add_in_orbit(&luna_data.mul_radius(0.1), earth);
            ship.set_in_orbit(masses, &ship_data.mul_radius(0.1), earth);
            ship.set_burn(0.289, 1.1781022706580768); // Luna-Orbit
            ship.set_burn(0.339, 1.48); // Not an 8 curse yet
            simulation.run_mode = false;
        }
    };

    // All masses are there, calculate the simulation time by the maximal orbit time
    simulation.set_orbit_time(&masses);
    masses.set_radius(canvas);

    // initially simulate all the future positinos
    masses.predict_positions(simulation);
    ship.predict_positions(simulation, masses);
}

#[macroquad::main(conf)]
async fn main() {
    let mut simulation = Simulation::new();
    let mut masses = Masses::new();
    let mut ship = Ship::default();
    let mut canvas = Canvas::new(&conf());

    set_masses(0, &mut simulation, &mut masses, &mut ship, &mut canvas);

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
                    simulation = Simulation::new();
                    set_masses(
                        0, //simulation.case,
                        &mut simulation,
                        &mut masses,
                        &mut ship,
                        &mut canvas,
                    )
                }
                '0' => {
                    simulation = Simulation::new();
                    set_masses(0, &mut simulation, &mut masses, &mut ship, &mut canvas)
                }
                '1' => {
                    simulation = Simulation::new();
                    set_masses(1, &mut simulation, &mut masses, &mut ship, &mut canvas)
                }
                '2' => {
                    simulation = Simulation::new();
                    set_masses(2, &mut simulation, &mut masses, &mut ship, &mut canvas)
                }
                '3' => {
                    simulation = Simulation::new();
                    set_masses(3, &mut simulation, &mut masses, &mut ship, &mut canvas)
                }
                '4' => {
                    simulation = Simulation::new();
                    set_masses(4, &mut simulation, &mut masses, &mut ship, &mut canvas)
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
                // also sets index to next step
                simulation.simulate_one_step(&mut masses);
            }

            // Predict ship with new masses index
            ship.predict_positions(&simulation, &masses);
            //simulation.predict_ship_positions();
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
