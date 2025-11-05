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
    let sun_data = MassData {
        name: "sun",
        color: YELLOW,
        diameter_km: 1.3914e6,
        mass: 1.989e30,
        px_ae: 0.0,
        py_ae: 0.0,
        vx_ae: 0.0,
        vy_ae: 0.0,
        orbits: None,
    };

    let sun = Mass::new(&sun_data);

    let earth_data = MassData {
        name: "earth",
        color: BLUE,
        diameter_km: 12756.32,
        mass: 5.974e24,
        px_ae: 0.0,
        py_ae: -1.0,
        vx_ae: 0.0,
        vy_ae: 0.0,
        orbits: Some(&sun),
    };

    let mut earth = Mass::new(&earth_data);

    let luna_data = MassData {
        name: "luna",
        color: RED,
        diameter_km: 3476.0,
        mass: 7.349e22,
        px_ae: 0.0,
        py_ae: -1.0002,
        vx_ae: 0.0,
        vy_ae: 0.0,
        orbits: Some(&earth),
    };

    let mut luna = Mass::new(&luna_data);

    let _jupiter_data = MassData {
        name: "jupiter",
        color: GREEN,
        diameter_km: 142984.0,
        mass: 1.899e27,
        px_ae: 0.0,
        py_ae: 1.0,
        vx_ae: 25e3,
        vy_ae: 0.0,
        orbits: Some(&sun),
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let delta_time: f64 = get_frame_time().into();

        const SECONDS_PER_YEAR: f64 = 60. * 60. * 24. * 365.;
        const SECONDS_PER_ORBIT: f64 = SECONDS_PER_YEAR / 10.;
        let seconds_per_frame = delta_time * SECONDS_PER_ORBIT;

        // simulation logic
        //simulation.move(seconds_per_frame);

        // draw / repaint the screen
        //simulation.repaint();

        clear_background(GRAY);
        draw_text("Hello, Macroquad", 20.0, 20.0, 30.0, DARKGRAY);

        earth.dragged_by(&sun);
        earth.dragged_by(&luna);
        luna.dragged_by(&earth);

        earth.frame_move(seconds_per_frame);
        luna.frame_move(seconds_per_frame);

        sun.draw();
        earth.draw();
        luna.draw();

        next_frame().await
    }
}
