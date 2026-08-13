mod mass;
mod vec_space;

// macroquad.rs
use macroquad::prelude::*;
use mass::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Gravity Sim Game"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn set_masses(case: i16) -> Masses {
    // some masses
    let sun_data = MassData::fixstar("sun", YELLOW, km(1.3914e6), mass_sol(1.));
    let sun2_data = MassData::orbiter("sun2", GOLD, km(1.3914e6), mass_sol(1.), au(0.5));
    let earth_data = MassData::orbiter("earth", BLUE, km(12756.32), mass_earth(1.), au(1.));

    // more but 0.005 AE radius makes the orbit insable.
    let luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
    let _jupiter_d = MassData::orbiter("jupiter", GREEN, km(142984.0), kg(1.899e27), au(25e3));
    let comet_data = MassData::ellipse("comet", WHITE, km(500.0), kg(1e6), au(1.3), 0.4);
    let ship_data = MassData::orbiter("ship", MAGENTA, km(10.0), kg(2e3), km(5000.));

    let mut masses = Masses::new(case);

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
            masses.set_text("Earth & Luna & Ship");
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
            masses.seconds_per_orbit = 50000.; // it is "per earth orbit"! Use maximal_orbit to ???
            let earth = masses.add_at_place(&earth_data);
            masses.add_in_orbit(&luna_data.multiplied_orbit_radius(0.1), earth);
            masses.add_in_orbit(&ship_data, earth);
        }
    };

    // let dt_sim =  SIM_TIME * SECONDS_PER_YEAR / SECONDS_PER_ORBIT;

    masses.simulated_seconds_per_secound = SECONDS_PER_YEAR / masses.seconds_per_orbit;
    masses.simulated_seconds_per_frame = SIM_TIME * masses.simulated_seconds_per_secound;

    masses.simulate_positions();

    masses
}

fn line_color(sub: i16) -> Color {
    const LIGHT: f32 = 0.3;
    const MEDIUM: f32 = 0.25;
    const DARK: f32 = 0.2;
    let mut a: f32 = DARK;
    if sub % 5 == 0 {
        if sub % 10 == 0 { a = LIGHT } else { a = MEDIUM }
    }
    Color {
        r: 0.,
        g: 1.,
        b: 0.,
        a,
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut masses = set_masses(1);

    let mut frame_delta_sum = 0.0;

    loop {
        if let Some(char) = get_char_pressed() {
            // println!("pressed char {:?}!", char);
            match char {
                '\u{1b}' => break, // KeyCode::Escape
                '\r' => {
                    // KeyCode::Enter
                    masses.toggle_planing_mode();
                    println!(
                        "planing_mode: {} {}",
                        masses.planing_mode, masses.simulated_seconds
                    );
                }

                'r' => masses = set_masses(masses.case),
                '0' => masses = set_masses(0),
                '1' => masses = set_masses(1),
                '2' => masses = set_masses(2),
                '3' => masses = set_masses(3),
                '4' => masses = set_masses(4),

                _ => println!("Char not used: {:?}!", char),
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

        clear_background(BLACK);

        // fn draw_lines, were to ???
        const CENTER_X: f32 = WINDOW_WIDTH / 2.;
        const CENTER_Y: f32 = WINDOW_HEIGHT / 2.;

        let step: f32 = CENTER_X / 30.;
        let mut x = -CENTER_X;
        let mut sub = 0;
        loop {
            draw_line(
                CENTER_X + x,
                0.0,
                CENTER_X + x,
                WINDOW_HEIGHT,
                1.,
                line_color(sub),
            );
            x += step;
            sub += 1;
            if x > CENTER_X {
                break;
            }
        }

        let mut x = -CENTER_Y;
        let mut sub = 0;
        loop {
            draw_line(
                0.0,
                CENTER_Y + x,
                WINDOW_WIDTH,
                CENTER_Y + x,
                1.,
                line_color(sub),
            );
            x += step;
            sub += 1;
            if x > CENTER_Y {
                break;
            }
        }

        masses.draw();

        // simulate next position
        let frame_delta_time: f64 = (get_frame_time() as f64).min(1.0);
        frame_delta_sum += frame_delta_time;

        // Simulate nothing or one ore some simulation steps
        while frame_delta_sum > SIM_TIME {
            frame_delta_sum -= SIM_TIME;
            masses.simulate_next_position();
        }

        next_frame().await
    }
}
