mod mass;
mod vecx;

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
    let sun_data = MassData {
        name: "sun",
        color: YELLOW,
        diameter: 1.3914e6 * 1000.,
        mass: 1.989e30,
        ..Default::default()
    };

    let sun2_data = MassData {
        name: "sun2",
        color: GOLD,
        diameter: 1.3914e6 * 1000.,
        mass: 1.989e30,
        radius: 0.5 * M_AE,
        ..Default::default()
    };

    let earth_data = MassData {
        name: "earth",
        color: BLUE,
        diameter: 12756.32 * 1000.,
        mass: 5.974e24,
        radius: 1.0 * M_AE,
        ..Default::default()
    };

    let luna_data = MassData {
        name: "luna",
        color: RED,
        diameter: 3476. * 1000.,
        mass: 7.349e22,
        radius: 370171. * 1000., // more but 0.005 AE makes the orbit insable.
        // todo: excentricity: real value,
        ..Default::default()
    };

    let _jupiter_data = MassData {
        name: "jupiter",
        color: GREEN,
        diameter: 142984.0 * 1000.,
        mass: 1.899e27,
        radius: 25e3 * M_AE,
        ..Default::default()
    };

    let comet_data = MassData {
        name: "comet",
        color: WHITE,
        diameter: 500.0 * 1000.,
        mass: 1e6,
        radius: 1.3 * M_AE,
        excentricity: 0.4,
        ..Default::default()
    };

    let mut masses = Masses::new();

    let text = match 1 {
        0 => {
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&sun2_data, sun);
            "double star"
        }

        _ => {
            let sun = masses.add_at_place(&sun_data);
            let earth = masses.add_in_orbit(&earth_data, sun);
            masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&comet_data, sun);
            "Sun, Earth and Moon"
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
        masses.draw();

        next_frame().await
    }
}
