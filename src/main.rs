mod canvas;
mod masses;
mod simulation;
mod vec_space;

use canvas::*;
use macroquad::prelude::*;
use masses::*;
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

fn set_masses(scenario: i16, canvas: &mut Canvas) -> Simulation {
    // some masses
    let sun_data = MassData::fixstar("sun", YELLOW, km(1.3914e6), mass_sol(1.));
    let sun2_data = MassData::orbiter("sun2", GOLD, km(1.3914e6), mass_sol(1.), au(0.5));
    let earth_data = MassData::orbiter("earth", BLUE, km(12756.32), mass_earth(1.), au(1.));

    // more but 0.005 AE radius makes the orbit insable.
    let luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
    let _jupiter_d = MassData::orbiter("jupiter", GREEN, km(142984.0), kg(1.899e27), au(25e3));
    let comet_data = MassData::ellipse("comet", WHITE, km(500.0), kg(1e6), au(1.3), 0.4);
    //let ship_data = MassData::orbiter("ship", WHITE, 10.0, 0.0, km(80000.)); // the real 300km are not visible

    let mut simulation = Simulation::new(scenario);

    match scenario {
        1 => {
            simulation.set_text("Sun, Earth");
            let sun = simulation.masses.add_mass_at_place(&sun_data);
            simulation.masses.add_mass_in_orbit(&earth_data, sun);
        }

        2 => {
            simulation.set_text("double star");
            let sun = simulation.masses.add_mass_at_place(&sun_data);
            simulation.masses.add_mass_in_orbit(&sun2_data, sun);
        }

        3 => {
            simulation.set_text("Earth & Luna & Ship");
            simulation.set_seconds_per_orbit(60.);
            let earth = simulation.masses.add_mass_at_place(&earth_data);
            simulation.masses.add_mass_in_orbit(&luna_data, earth);
            //simulation.masses.add_ship_in_orbit(&ship_data, earth);
        }

        4 => {
            simulation.set_text("Sun, Earth & Luna");
            let sun = simulation.masses.add_mass_at_place(&sun_data);
            let earth = simulation.masses.add_mass_in_orbit(&earth_data, sun);
            simulation.masses.add_mass_in_orbit(&luna_data, earth);
            simulation.masses.add_mass_in_orbit(&comet_data, sun);
        }

        _ => {
            simulation.set_text("Test");
            let earth = simulation.masses.add_mass_at_place(&earth_data);
            simulation
                .masses
                .add_mass_in_orbit(&luna_data.multiplied_orbit_radius(0.1), earth);
            //simulation.add_ship_in_orbit(&ship_data.multiplied_orbit_radius(0.1), earth);

            simulation.start_time = 0.339;
            simulation.burn_time = 1.48;
            simulation.run_mode = true;
        }
    };

    // All masses are there, calculate the simulation time by the maximal orbit time
    simulation.set_orbit_time();
    simulation.masses.set_radius(canvas);

    simulation.predict_positions();
    //simulation.predict_ship_positions();
    simulation
}

#[macroquad::main(conf)]
async fn main() {
    let mut canvas = Canvas::new(&conf());
    let mut simulation = set_masses(0, &mut canvas);

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

                'r' => simulation = set_masses(simulation.case, &mut canvas),
                '0' => simulation = set_masses(0, &mut canvas),
                '1' => simulation = set_masses(1, &mut canvas),
                '2' => simulation = set_masses(2, &mut canvas),
                '3' => simulation = set_masses(3, &mut canvas),
                '4' => simulation = set_masses(4, &mut canvas),

                _ => (), // println!("Char not used: {:?}!", char),
            }
        }

        clear_background(BLACK);

        canvas.draw_grid();

        simulation.draw(&canvas);

        // simulate next position to be drawn in the next loop
        let frame_delta_time: f64 = (get_frame_time() as f64).min(1.0);
        frame_delta_sum += frame_delta_time;

        // Simulate nothing or one ore some simulation steps
        key_down(&mut simulation, &mut canvas, SIMULATION_STEP_TIME);

        while frame_delta_sum > SIMULATION_STEP_TIME {
            frame_delta_sum -= SIMULATION_STEP_TIME;

            if simulation.run_mode {
                simulation.simulate_one_step();
                // set index to next step?
            }
            // Predict ship with new index
            //simulation.predict_ship_positions();
        }

        next_frame().await
    }
}

fn key_down(simulation: &mut Simulation, canvas: &mut Canvas, _simulation_step_time: f64) {
    if is_key_down(KeyCode::Space) {
        //masses.ship_accelerate(simulation_step_time);
    }
    if is_key_down(KeyCode::Backspace) {
        //masses.ship_accelerate(-simulation_step_time)
    }
    if is_key_down(KeyCode::Right) {
        simulation.planing_start_time(1.);
    }
    if is_key_down(KeyCode::Left) {
        simulation.planing_start_time(-1.);
    }
    if is_key_down(KeyCode::Up) {
        simulation.planing_burn_time(1.);
    }
    if is_key_down(KeyCode::Down) {
        simulation.planing_burn_time(-1.);
    }

    if is_key_down(KeyCode::U) {
        canvas.mul_z_view(1.001);
    }
    if is_key_down(KeyCode::J) {
        canvas.mul_z_view(0.999);
    }
}
