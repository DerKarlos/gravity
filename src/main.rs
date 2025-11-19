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

#[macroquad::main(conf)]
async fn main() {
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
    let _jupiter_d = MassData::orbiter(
        "jupiter",
        GREEN,
        Si::Km(142984.0),
        Si::Kg(1.899e27),
        Si::Au(25e3),
    );
    let comet_data =
        MassData::ellipse("comet", WHITE, Si::Km(500.0), Si::Kg(1e6), Si::Au(1.3), 0.4);
    let ship_data = MassData::orbiter("ship", MAGENTA, Si::Km(10.0), Si::Kg(2e3), Si::Km(20000.));

    let mut masses = Masses::new();
    let mut seconds_per_orbit: f64 = 10.;

    let text = match 3 {
        1 => {
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&earth_data, sun);
            "Sun, Earth"
        }

        2 => {
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&sun2_data, sun);
            "double star"
        }

        3 => {
            seconds_per_orbit = 20000.;
            let earth = masses.add_at_place(&earth_data);
            //masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&ship_data, earth);
            "Earth & Ship & Luna"
        }

        _ => {
            let sun = masses.add_at_place(&sun_data);
            let earth = masses.add_in_orbit(&earth_data, sun);
            masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&comet_data, sun);
            "Sun, Earth & Luna"
        }
    };

    //println!("{}", seconds_per_orbit);
    //seconds_per_orbit = 1e19 / masses.maximal_orbit / masses.maximal_orbit;
    //println!("{}", seconds_per_orbit);

    const FRAME_TIME: f64 = 0.02;
    let simulated_seconds_per_secound: f64 = SECONDS_PER_YEAR / seconds_per_orbit;
    let simulated_seconds_per_frame = FRAME_TIME * simulated_seconds_per_secound;

    let mut simulated_seconds = 0.0;
    let mut frame_seconds = 0.0;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::Space) {
            masses.ship_accelerate(1.0)
        }
        if is_key_down(KeyCode::Backspace) {
            masses.ship_accelerate(-1.0)
        }

        let delta_time: f64 = (get_frame_time() as f64).min(1.0);
        frame_seconds += delta_time;
        if frame_seconds < FRAME_TIME {
            continue;
        }

        while frame_seconds > FRAME_TIME {
            frame_seconds -= FRAME_TIME;

            // simulation logic and drawing
            masses.simulate(simulated_seconds_per_frame, simulated_seconds);
            masses.predict(simulated_seconds_per_frame, simulated_seconds);
            simulated_seconds += simulated_seconds_per_frame;
        }

        // draw_text(
        //     (simulated_seconds / SECONDS_PER_DAY).to_string().as_str(),
        //     20.0,
        //     40.0,
        //     30.0,
        //     DARKGRAY,
        // );

        clear_background(GRAY);
        draw_text(text, 20.0, 20.0, 30.0, DARKGRAY);
        masses.draw();

        next_frame().await
    }
}
