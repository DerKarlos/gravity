mod mass;

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
        diameter_km: 1.3914e6,
        mass: 1.989e30,
        ..Default::default()
    };

    let sun2_data = MassData {
        name: "sun2",
        color: GOLD,
        diameter_km: 1.3914e6,
        mass: 1.989e30,
        radius: 0.5,
        ..Default::default()
    };

    let earth_data = MassData {
        name: "earth",
        color: BLUE,
        diameter_km: 12756.32,
        mass: 5.974e24,
        radius: 1.0,
        ..Default::default()
    };

    let _luna_data = MassData {
        name: "luna",
        color: RED,
        diameter_km: 3476.0,
        mass: 7.349e22,
        radius: -0.05,
        ..Default::default()
    };

    let _jupiter_data = MassData {
        name: "jupiter",
        color: GREEN,
        diameter_km: 142984.0,
        mass: 1.899e27,
        radius: 25e3,
        ..Default::default()
    };

    let comet_data = MassData {
        name: "comet",
        color: WHITE,
        diameter_km: 500.0,
        mass: 1e6,
        radius: 1.3,
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
            let _earth = masses.add_in_orbit(&earth_data, sun);
            //masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&comet_data, sun);
            "Sun & Earth"
        }
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let mut delta_time: f64 = get_frame_time().into();
        if delta_time > 0.1 {
            delta_time = 0.1
        };

        const SECONDS_PER_YEAR: f64 = 60. * 60. * 24. * 365.;
        const SECONDS_PER_ORBIT: f64 = SECONDS_PER_YEAR / 10.;
        let seconds_per_frame = delta_time * SECONDS_PER_ORBIT;

        clear_background(GRAY);
        draw_text(text, 20.0, 20.0, 30.0, DARKGRAY);

        // simulation logic and drawing
        masses.frame(seconds_per_frame);

        next_frame().await
    }
}
