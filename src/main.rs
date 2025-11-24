mod mass;
mod vec_space;

// macroquad.rs
use macroquad::prelude::*;
use mass::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Gravity Sim Game"),
        window_width: 1000,
        window_height: 708,
        window_resizable: false,
        ..Default::default()
    }
}

fn set_masses(case: i16) -> Masses {
    // some masses
    let sun_data = MassData::fixstar("sun", YELLOW, Si::Km(1.3914e6), Si::Sol(1.));
    let sun2_data = MassData::orbiter("sun2", GOLD, Si::Km(1.3914e6), Si::Sol(1.), Si::Au(0.5));
    let earth_data = MassData::orbiter("earth", BLUE, Si::Km(12756.32), Si::Earth(1.), Si::Au(1.));

    // more but 0.005 AE radius makes the orbit insable.
    let luna_data = MassData::orbiter(
        "luna",
        RED,
        Si::Km(3476.),
        Si::Kg(7.349e22),
        Si::Km(370171.),
    );
    let near_moon_data = MassData::orbiter(
        "near_moon",
        RED,
        Si::Km(3476.),
        Si::Kg(7.349e22),
        Si::Km(370171. / 10.),
    );
    let _jupiter_d = MassData::orbiter(
        "jupiter",
        GREEN,
        Si::Km(142984.0),
        Si::Kg(1.899e27),
        Si::Au(25e3),
    );
    let comet_data =
        MassData::ellipse("comet", WHITE, Si::Km(500.0), Si::Kg(1e6), Si::Au(1.3), 0.4);
    let ship_data = MassData::orbiter("ship", MAGENTA, Si::Km(10.0), Si::Kg(2e3), Si::Km(5000.));

    let mut masses = Masses::new();

    match case {
        1 => {
            masses.set_text("Sun, Earth");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&earth_data, sun);
        }

        2 => {
            masses.set_text("double star");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&sun2_data, sun);
        }

        3 => {
            masses.set_text("Earth & Ship & Luna");
            masses.seconds_per_orbit = 2000.;
            let earth = masses.add_at_place(&earth_data);
            masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&ship_data, earth);
        }

        4 => {
            masses.set_text("Sun, Earth & Luna");
            let sun = masses.add_at_place(&sun_data);
            let earth = masses.add_in_orbit(&earth_data, sun);
            masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&comet_data, sun);
        }

        _ => {
            masses.set_text("Test");
            masses.seconds_per_orbit = 50000.;
            let earth = masses.add_at_place(&earth_data);
            let mut moon_data = luna_data;
            moon_data.mul_orbit_radius(0.1);
            masses.add_in_orbit(&near_moon_data, earth);
            masses.add_in_orbit(&ship_data, earth);
        }
    };

    masses

    //println!("{}", seconds_per_orbit);
    //seconds_per_orbit = 1e19 / masses.maximal_orbit / masses.maximal_orbit;
    //println!("{}", seconds_per_orbit);
}

#[macroquad::main(conf)]
async fn main() {
    let case = 4;
    let mut masses = set_masses(case);

    const FRAME_TIME: f64 = 0.02; // 20ms = 50Hz
    let simulated_seconds_per_secound: f64 = SECONDS_PER_YEAR / masses.seconds_per_orbit;
    let simulated_seconds_per_frame = FRAME_TIME * simulated_seconds_per_secound / MYSTIC_G_FACT;

    let mut simulated_seconds = 0.0;
    let mut frame_delta_sum = 0.0;
    let mut fix_pressed = 0; // is_key_pressed needs about 4 loops to stop a true
    // todo: is_key_pressed does not fire once

    loop {
        if fix_pressed > 0 {
            fix_pressed -= 1;
        } else {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            if is_key_pressed(KeyCode::R) {
                fix_pressed = 50; // 5 is not enoung to avoid a hang up
                simulated_seconds = 0.0;
                frame_delta_sum = 0.0;
                masses = set_masses(case);
                println!("set_masses");
            }

            if is_key_pressed(KeyCode::L) {
                fix_pressed = 5;
                masses.toggle_planing_mode(simulated_seconds);
                println!(
                    "planing_mode: {} {}",
                    masses.planing_mode, simulated_seconds
                );
            }
        }

        if is_key_down(KeyCode::Space) {
            masses.ship_accelerate(1.0);
        }
        if is_key_down(KeyCode::Backspace) {
            masses.ship_accelerate(-1.0)
        }

        if is_key_down(KeyCode::Right) {
            masses.planing_start_time(1.);
        }
        if is_key_down(KeyCode::Left) {
            masses.planing_start_time(-1.);
        }
        if is_key_down(KeyCode::Up) {
            masses.planing_burn_time(1.);
        }
        if is_key_down(KeyCode::Down) {
            masses.planing_burn_time(-1.);
        }

        let frame_delta_time: f64 = (get_frame_time() as f64).min(1.0);
        frame_delta_sum += frame_delta_time;
        if frame_delta_sum < FRAME_TIME {
            continue;
        }

        while frame_delta_sum > FRAME_TIME {
            frame_delta_sum -= FRAME_TIME;

            // simulation logic and drawing
            if !masses.planing_mode {
                masses.simulate(simulated_seconds_per_frame, simulated_seconds);
                simulated_seconds += simulated_seconds_per_frame;
            }
            masses.predict(simulated_seconds_per_frame, simulated_seconds);
        }

        clear_background(GRAY);
        masses.draw();

        next_frame().await
    }
}
