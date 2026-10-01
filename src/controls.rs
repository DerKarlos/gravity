use crate::canvas::*;
use crate::masses::*;
use crate::ship::*;
use crate::simulation::*;

use macroquad::prelude::*;

const KEY_BREAK_TIME: f64 = 0.33;

pub struct Controls {
    x_view: UpDown,
    y_view: UpDown,
    z_view: UpDown,
    start_time: UpDown,
    burn_time: UpDown,
}

impl Controls {
    pub fn new(ship: &Ship) -> Controls {
        Controls {
            x_view: UpDown::new("x_view", Formular::ADD, 0., -9999., 50., 1.), // value, min, step, min
            y_view: UpDown::new("y_view", Formular::ADD, 0., -9999., 50., 1.),
            z_view: UpDown::new("z_view", Formular::ADD, 1., 0.0001, 1., 1.),
            start_time: UpDown::new("start", Formular::MUL, ship.burn_start, 0.0001, 1., 0.001),
            burn_time: UpDown::new("burn", Formular::MUL, ship.burn_time, 0.0001, 1., 0.001),
        }
    }

    pub fn key_down(
        &mut self,
        ship: &mut Ship,
        masses: &mut Masses,
        canvas: &mut Canvas,
        delta_time: f64,
    ) {
        if is_key_down(KeyCode::Space) {
            ship.mass.ship_accelerate_ahead(SIMULATION_STEP_TIME);
        }
        if is_key_down(KeyCode::Backspace) {
            ship.mass.ship_accelerate_ahead(-SIMULATION_STEP_TIME)
        }

        ship.set_start_time(
            self.start_time
                .up_down(KeyCode::Right, KeyCode::Left, delta_time),
        );

        ship.set_burn_time(
            self.burn_time
                .up_down(KeyCode::Up, KeyCode::Down, delta_time),
        );

        if is_key_down(KeyCode::O) {
            masses.mul_predict_count(1.003);
        }
        if is_key_down(KeyCode::L) {
            masses.mul_predict_count(0.996);
        }

        if is_key_down(KeyCode::I) {
            masses.ramp_predict_show(true);
        }
        if is_key_down(KeyCode::K) {
            masses.ramp_predict_show(false);
        }

        canvas.draw_set_x_view(self.x_view.up_down(KeyCode::A, KeyCode::D, delta_time));
        canvas.draw_set_y_view(self.y_view.up_down(KeyCode::W, KeyCode::S, delta_time));
        canvas.draw_set_z_view(self.z_view.up_down(KeyCode::E, KeyCode::Q, delta_time));

        // + RightBracket
        // - Apostrophe
    }
}

#[derive(Clone, Copy)]
enum Formular {
    ADD,
    MUL,
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
            println!("{} up: {}", self.name, set);
        }
        if is_key_down(key_down) {
            set = self.down(delta_time);
            println!("{} down: {}", self.name, set);
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
            Formular::MUL => self.actual *= 1. + 1. * delta_time * self.step,
            Formular::ADD => self.actual += delta_time * self.step,
        }

        self.actual
    }
    fn down(&mut self, delta_time: f64) -> f64 {
        let time = get_time();
        if time - self.last_time > KEY_BREAK_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            self.last_was_up = false;
            self.step_down();
        } else {
            // self.step *= 1.01;
            self.step *= 1. + 1. * delta_time;
        }

        // println!("dn {} - {}", self.actual, self.step);
        // self.actual *= 1. - 1. * delta_time * self.step;
        match self.formular {
            Formular::MUL => self.actual *= 1. - 1. * delta_time * self.step,
            Formular::ADD => self.actual -= delta_time * self.step,
        }

        if self.actual < self.actual_min {
            self.actual = self.actual_min;
        }
        self.actual
    }
}
