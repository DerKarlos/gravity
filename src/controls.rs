use crate::canvas::*;
use crate::masses::*;
use crate::ship::*;
use crate::simulation::*;
// for keyboard
use macroquad::prelude::*;

const KEY_BREAK_TIME: f64 = 0.33; // ui-time

pub struct Controls {
    x_view: UpDown,
    y_view: UpDown,
    z_view: UpDown,
    predict_show: UpDown,
    start_time: UpDown, // ??? change to name _seconds and fuction
    burn_time: UpDown,
    burn_acceleration: UpDown,
}

impl Controls {
    pub fn new(ship: &Ship) -> Controls {
        Controls {
            x_view: UpDown::new("x_view", Formular::Add, 0., -9999., 50., 1.), // value, min, step, min
            y_view: UpDown::new("y_view", Formular::Add, 0., -9999., 50., 1.),
            z_view: UpDown::new("z_view", Formular::Add, 0.95, 0.0001, 1., 1.),
            predict_show: UpDown::new("p_show", Formular::Add, 0., 0., 10., 1.),
            start_time: UpDown::new("b_start", Formular::Mul, ship.burn_start, 0.0001, 1., 0.001),
            burn_time: UpDown::new("b_time", Formular::Mul, ship.burn_time, 0.0000, 1., 0.001), // min = 0 = off
            burn_acceleration: UpDown::new(
                "power",
                Formular::Mul,
                ship.burn_acceleration,
                0.0001,
                1.,
                0.001,
            ),
        }
    }

    pub fn key_down(&mut self, ship: &mut Ship, masses: &mut Masses, delta_time: f64) {
        if is_key_down(KeyCode::Space) {
            ship.mass
                .ship_accelerate_ahead(SIMULATION_STEP_SECONDS * ship.burn_acceleration);
        }
        if is_key_down(KeyCode::Backspace) {
            ship.mass
                .ship_accelerate_ahead(-SIMULATION_STEP_SECONDS * ship.burn_acceleration)
        }

        ship.set_start_time(
            self.start_time
                .up_down(KeyCode::Right, KeyCode::Left, delta_time), // ui-time
        );

        ship.set_burn_time(
            self.burn_time
                .up_down(KeyCode::Up, KeyCode::Down, delta_time),
        );

        ship.set_burn_acceleration(self.burn_acceleration.up_down(
            KeyCode::RightBracket,
            KeyCode::Apostrophe,
            delta_time,
        ));

        if is_key_down(KeyCode::O) {
            masses.mul_predict_count(1.003);
        }
        if is_key_down(KeyCode::L) {
            masses.mul_predict_count(0.996);
        }

        masses.set_predict_show(
            self.predict_show
                .up_down(KeyCode::I, KeyCode::K, delta_time),
        );

        draw_set_view(
            self.x_view.up_down(KeyCode::A, KeyCode::D, delta_time),
            self.y_view.up_down(KeyCode::W, KeyCode::S, delta_time),
            self.z_view.up_down(KeyCode::E, KeyCode::Q, delta_time),
        );
    }
}

#[derive(Clone, Copy)]
enum Formular {
    Add,
    Mul,
}

struct UpDown {
    name: &'static str,
    formular: Formular,
    last_time: f64,
    actual: f64,
    actual_min: f64,
    step: f64,
    step_min: f64,
    last_was_up: bool,
}

impl UpDown {
    pub fn new(
        name: &'static str,
        formular: Formular,
        actual: f64,
        actual_min: f64,
        step: f64,
        step_min: f64,
    ) -> UpDown {
        UpDown {
            name,
            formular,
            last_time: 0.,
            actual,
            actual_min,
            step,
            step_min,
            last_was_up: true,
        }
    }

    fn step_down(&mut self) {
        self.step /= 2.;
        if self.step < self.step_min {
            self.step = self.step_min;
        }
        println!("step_down {}", self.step);
    }

    pub fn up_down(&mut self, key_up: KeyCode, key_down: KeyCode, delta_time: f64) -> f64 {
        let mut set = self.actual;
        if is_key_down(key_up) {
            set = self.up(delta_time);
            println!("{} up: {} / {}", self.name, set, self.step);
        }
        if is_key_down(key_down) {
            set = self.down(delta_time);
            println!("{} down: {} / {}", self.name, set, self.step);
        }
        set
    }

    fn up(&mut self, delta_time: f64) -> f64 {
        let time = get_time();
        if time - self.last_time > KEY_BREAK_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            // 1 sec to double the value! If dt is 1 it doubles beause 1+1*1=2. If dt is smal, less happends
            self.step *= 1. + 1. * delta_time;
        } else {
            self.last_was_up = true;
            self.step_down();
        }

        // println!("up {} - {}", self.actual, self.step);
        // 1 sec to double the value! If dt is 1 it doubles beause 1+1*1=2. If dt is smal, less happends

        match self.formular {
            Formular::Mul => self.actual *= 1. + 1. * delta_time * self.step,
            Formular::Add => self.actual += delta_time * self.step,
        }

        self.actual
    }
    fn down(&mut self, delta_time: f64) -> f64 {
        let last_step = self.step;
        let time = get_time();
        if time - self.last_time > KEY_BREAK_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            self.last_was_up = false;
            self.step_down();
        } else {
            self.step *= 1. + 1. * delta_time;
        }

        // println!("dn {} - {}", self.actual, self.step);
        // self.actual *= 1. - 1. * delta_time * self.step;
        match self.formular {
            Formular::Mul => self.actual *= 1. - 1. * delta_time * self.step,
            Formular::Add => self.actual -= delta_time * self.step,
        }

        if self.actual < self.actual_min {
            self.actual = self.actual_min;
            self.step = last_step;
        }

        self.actual
    }
}
