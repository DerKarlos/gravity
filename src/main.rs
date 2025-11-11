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
    let sun_data = MassData::fixstar("sun", YELLOW, 1.3914e6 * 1000., 1.989e30);
    let sun2_data = MassData::orbiter("sun2", GOLD, 1.3914e6 * 1000., 1.989e30, 0.5 * M_AE);
    let earth_data = MassData::orbiter("earth", BLUE, 12756.32 * 1000., 5.974e24, 1.0 * M_AE);
    let luna_data = MassData::orbiter("luna", RED, 3476. * 1000., 7.349e22, 370171. * 1000.); // more but 0.005 AE radius makes the orbit insable.
    let _jupiter_data =
        MassData::orbiter("jupiter", GREEN, 142984.0 * 1000., 1.899e27, 25e3 * M_AE);
    let comet_data = MassData::ellipse("comet", WHITE, 500.0 * 1000., 1e6, 1.3 * M_AE, 0.4);

    let mut masses = Masses::new();

    let text = match 0 {
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

        _ => {
            let sun = masses.add_at_place(&sun_data);
            let earth = masses.add_in_orbit(&earth_data, sun);
            masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&comet_data, sun);
            "Sun, Earth & Moon"
        }
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let delta_time: f64 = (get_frame_time() as f64).min(0.1);

        //nst SECONDS_PER_ORBIT: f64 = 10.;
        const SECONDS_PER_YEAR: f64 = 60. * 60. * 24. * 365.;
        const SECONDS_PER_ORBIT: f64 = SECONDS_PER_YEAR / 10.;
        let seconds_per_frame = delta_time * SECONDS_PER_ORBIT;

        clear_background(GRAY);
        draw_text(text, 20.0, 20.0, 30.0, DARKGRAY);

        // simulation logic and drawing
        masses.frame(seconds_per_frame);
        masses.predict(seconds_per_frame);
        masses.draw();

        next_frame().await
    }
}
