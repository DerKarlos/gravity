use crate::canvas::*;
use crate::ship::*;

use macroquad::prelude::*;

const KEY_BREAK_TIME: f64 = 0.33;

pub struct Controls {
    z_view: UpDown,
    start_time: UpDown,
    burn_time: UpDown,
}

impl Controls {
    pub fn new(ship: &Ship) -> Controls {
        Controls {
            z_view: UpDown::new(1., 0.0001, 1., 1.), // value, step, min
            start_time: UpDown::new(ship.burn_start, 0.0001, 1., 0.001),
            burn_time: UpDown::new(5., 0.0001, 1., 0.001),
        }
    }

    pub fn key_down(&mut self, ship: &mut Ship, canvas: &mut Canvas, delta_time: f64) {
        let time = get_time();

        if is_key_down(KeyCode::Space) {
            //masses.ship_accelerate(simulation_step_time);
        }
        if is_key_down(KeyCode::Backspace) {
            //masses.ship_accelerate(-simulation_step_time)
        }
        if is_key_down(KeyCode::Right) {
            //ship.planing_start_time(1.);
            ship.set_start_time(self.start_time.up(time, delta_time));
        }
        if is_key_down(KeyCode::Left) {
            //ship.planing_start_time(-1.);
            ship.set_start_time(self.start_time.down(time, delta_time));
        }
        if is_key_down(KeyCode::Up) {
            ship.planing_burn_time(1.);
        }
        if is_key_down(KeyCode::Down) {
            ship.planing_burn_time(-1.);
        }

        if is_key_down(KeyCode::O) {
            canvas.mul_predict_count(1.003);
        }
        if is_key_down(KeyCode::L) {
            canvas.mul_predict_count(0.996);
        }

        if is_key_down(KeyCode::I) {
            canvas.ramp_predict_show(true);
        }
        if is_key_down(KeyCode::K) {
            canvas.ramp_predict_show(false);
        }

        if is_key_down(KeyCode::W) {
            canvas.add_view(0., 1.);
        }
        if is_key_down(KeyCode::S) {
            canvas.add_view(0., -1.);
        }

        if is_key_down(KeyCode::A) {
            canvas.add_view(1., 0.);
        }
        if is_key_down(KeyCode::D) {
            canvas.add_view(-1., 0.);
        }

        if is_key_down(KeyCode::E) {
            //canvas.mul_z_view(1.001);
            canvas.set_z_view(self.z_view.up(time, delta_time));
        }
        if is_key_down(KeyCode::Q) {
            //canvas.mul_z_view(0.999);
            canvas.set_z_view(self.z_view.down(time, delta_time));
        }

        //let x = get_keys_pressed();
        //if x.len() > 0 {
        //    println!("keys: {:?}", x);
        //}

        // + RightBracket
        // - Apostrophe
    }
}

struct UpDown {
    last_time: f64,
    actual: f64,
    actual_min: f64,
    step: f64,
    step_min: f64,
    last_was_up: bool,
}

impl UpDown {
    pub fn new(actual: f64, actual_min: f64, step: f64, step_min: f64) -> UpDown {
        UpDown {
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

    pub fn up(&mut self, time: f64, delta_time: f64) -> f64 {
        if time - self.last_time > KEY_BREAK_TIME {
            self.step_down();
        }
        self.last_time = time;
        if self.last_was_up {
            // self.step *= 1.01;
            // 1 sec to double the value! If dt is 1 it doubles beause 1+1*1=2. If dt is smal, less happends
            self.step *= 1. + 1. * delta_time;
        } else {
            self.last_was_up = true;
            self.step_down();
        }

        // println!("up {} - {}", self.actual, self.step);
        // 1 sec to double the value! If dt is 1 it doubles beause 1+1*1=2. If dt is smal, less happends
        self.actual *= 1. + 1. * delta_time * self.step;
        self.actual
    }
    pub fn down(&mut self, time: f64, delta_time: f64) -> f64 {
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
        self.actual *= 1. - 1. * delta_time * self.step;
        if self.actual < self.actual_min {
            self.actual = self.actual_min;
        }
        self.actual
    }
}
